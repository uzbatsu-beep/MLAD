use std::net::SocketAddr;
use tokio::net::UdpSocket;
use suppftp::AsyncFtpStream;
use anyhow::{Result, anyhow};
use tracing::{info, error};

/// Модуль для работы со спецификацией High-Speed Ethernet (HSE) Yaskawa
pub struct YaskawaHseClient {
    target_addr: SocketAddr,
    socket: UdpSocket,
}

impl YaskawaHseClient {
    /// Инициализирует UDP сокет для связи с контроллером робота
    pub async fn new(robot_ip: &str, port: u16) -> Result<Self> {
        let target_addr: SocketAddr = format!("{}:{}", robot_ip, port).parse()?;
        // Привязываемся к любому свободному локальному порту
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect(target_addr).await?;
        
        info!("HSE Клиент успешно инициализирован для контроллера: {}", target_addr);
        Ok(Self { target_addr, socket })
    }

    /// Формирует бинарный заголовок YERC (32 байта) согласно спецификации hse_protocol.md
    fn build_yerc_header(&self, data_len: u16, request_id: u8) -> Vec<u8> {
        let mut header = Vec::with_capacity(32);
        
        header.extend_from_slice(b"YERC");               // Магическая константа (0-3)
        header.extend_from_slice(&32u16.to_le_bytes());  // Длина заголовка (4-5)
        header.extend_from_slice(&data_len.to_le_bytes());// Длина тела данных (6-7)
        header.push(1);                                  // Тип запроса: 1 = Чтение (8)
        header.push(request_id);                         // ID пакета для идентификации (9)
        header.extend_from_slice(&[0u8; 22]);            // Резерв и выравнивание Padding (10-31)
        
        header
    }

    /// Запрашивает состояние байта реле по его адресу из памяти Concurrent I/O
    pub async fn read_relay_byte(&self, io_class: u8, start_address: u16, request_id: u8) -> Result<u8> {
        // Тело команды чтения (4 байта): Код команды (0x03), класс, адрес
        let mut command_body = vec![0x03, io_class];
        command_body.extend_from_slice(&start_address.to_le_bytes());

        // Сборка полного пакета: Заголовок + Тело
        let mut packet = self.build_yerc_header(command_body.len() as u16, request_id);
        packet.extend(command_body);

        // Отправка пакета на робота
        self.socket.send(&packet).await?;

        // Ожидание ответа
        let mut buffer = [0u8; 64];
        let amt = self.socket.recv(&mut buffer).await?;

        if amt < 32 {
            return Err(anyhow!("Получен некорректный или усеченный пакет от робота."));
        }

        // Проверка статуса в ответе (обычно байт по смещению 8 или в теле ответа)
        if buffer[0..4] != *b"YERC" {
            return Err(anyhow!("Неверная сигнатура заголовка ответа робота."));
        }

        // Возвращаем первый байт данных из тела ответа (смещение 32)
        if amt >= 33 {
            Ok(buffer[32])
        } else {
            Err(anyhow!("Тело ответа не содержит данных о состоянии реле."))
        }
    }
}

/// Модуль автоматизации развертывания логики ПЛК по протоколу FTP
pub struct RobotFtpDeployer {
    robot_ip: String,
}

impl RobotFtpDeployer {
    pub fn new(robot_ip: &str) -> Self {
        Self { robot_ip: robot_ip.to_string() }
    }

    /// Выгружает скомпилированный текстовый файл листинга .LST на FTP сервер робота
    pub async fn upload_ladder_file(&self, local_file_path: &str, remote_filename: &str) -> Result<()> {
        info!("Подключение к FTP серверу робота {}:21 для деплоя...", self.robot_ip);
        
        // По стандарту Yaskawa использует анонимный вход или фиксированные учетные данные для наладчиков
        let mut ftp_stream = AsyncFtpStream::connect(format!("{}:21", self.robot_ip)).await?;
        ftp_stream.login("anonymous", "antigravity@ide.com").await?;
        
        // Переходим в директорию хранения программ параллельного ввода-вывода (может отличаться от версии контроллера)
        ftp_stream.cwd("LOGIC").await?;
        
        // Чтение локального скомпилированного файла
        let file_content = tokio::fs::read(local_file_path).await?;
        
        // Запись файла в контроллер робота
        ftp_stream.put_file(remote_filename, &mut file_content.as_slice()).await?;
        info!("Файл логики {} успешно загружен в память контроллера робота.", remote_filename);
        
        ftp_stream.quit().await?;
        Ok(())
    }
}
