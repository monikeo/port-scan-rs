use std::time::Duration;

const CONNECTION_TIME_MILLIS_SECOND: u64 = 1500;
const BANNER_SCANNING_TIME_MILLIS_SECOND: u64 = 5000;

pub async fn tcp_scan(ip: &str, port: u16) -> std::io::Result<()> {
    let addr = format!("{}:{}", ip, port).parse().unwrap();
    let socket = tokio::net::TcpSocket::new_v4()?;

    match tokio::time::timeout(
        Duration::from_millis(CONNECTION_TIME_MILLIS_SECOND),
        socket.connect(addr),
    )
    .await
    {
        Ok(result) => match result {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => return Err(error.into()),
        },
        Err(error) => {
            return Err(error.into());
        }
    }
}

pub async fn tcp_scan_ports(ip: &str, ports: &[u16]) -> Vec<u16> {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    let mut handles = Vec::new();
    let streams = Arc::new(Mutex::new(Vec::new()));
    for i in ports.iter().copied() {
        let ip = ip.to_string();
        let share_stream = streams.clone();
        let handle = tokio::spawn(async move {
            let mut lock = share_stream.lock().await;
            let stream = tcp_scan(&ip, i).await;
            if stream.is_ok() {
                lock.push(i);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let streams = Arc::try_unwrap(streams).expect("[-] ERROR: Mutext still have multiple owners");
    let streams = streams.into_inner();
    streams
}

pub async fn grab_banner(ip: &str, port: u16) -> std::io::Result<String> {
    use tokio::io::AsyncReadExt;
    let addr = format!("{}:{}", ip, port);
    let mut stream = tokio::net::TcpStream::connect(addr).await?;
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(banner)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_grab_banner() {
        let ip = "198.23.50.94";
        let port: u16 = 21;
        let result = grab_banner(ip, port).await;
    }
}
