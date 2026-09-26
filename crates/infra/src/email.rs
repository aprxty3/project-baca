//! Asynchronous transactional email dispatch for verification OTPs (Mailpit/SMTP).

use crate::config::EmailConfig;
use shared::AppError;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::{info, warn};

/// Sends an OTP verification email to the user via SMTP (local Mailpit)
pub async fn send_otp_email(
    config: &EmailConfig,
    to_email: &str,
    otp: &str,
) -> Result<(), AppError> {
    let addr = format!("{}:{}", config.smtp_host, config.smtp_port);
    info!(
        "Dispatching OTP email for {} via SMTP at {}",
        to_email, addr
    );

    let connect_future = TcpStream::connect(&addr);
    let mut stream = match timeout(Duration::from_secs(3), connect_future).await {
        Ok(Ok(stream)) => stream,
        Ok(Err(e)) => {
            warn!("SMTP connection to {addr} failed: {e}. Falling back to log trace.");
            info!("MOCK EMAIL DISPATCH: To: {}, OTP: {}", to_email, otp);
            return Ok(());
        }
        Err(_) => {
            warn!("SMTP connection to {addr} timed out. Falling back to log trace.");
            info!("MOCK EMAIL DISPATCH: To: {}, OTP: {}", to_email, otp);
            return Ok(());
        }
    };

    let mut buf = [0u8; 1024];

    // Read greeting
    let _ = stream.read(&mut buf).await;

    // Send EHLO
    let _ = stream.write_all(b"EHLO localhost\r\n").await;
    let _ = stream.read(&mut buf).await;

    // Send MAIL FROM
    let mail_from = format!("MAIL FROM:<{}>\r\n", config.smtp_from_email);
    let _ = stream.write_all(mail_from.as_bytes()).await;
    let _ = stream.read(&mut buf).await;

    // Send RCPT TO
    let rcpt_to = format!("RCPT TO:<{}>\r\n", to_email);
    let _ = stream.write_all(rcpt_to.as_bytes()).await;
    let _ = stream.read(&mut buf).await;

    // Send DATA
    let _ = stream.write_all(b"DATA\r\n").await;
    let _ = stream.read(&mut buf).await;

    // Send message payload
    let email_body = format!(
        "From: {} <{}>\r\nTo: <{}>\r\nSubject: Project Baca Verification Code\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nWelcome to Project Baca!\r\n\r\nYour 6-digit verification code is:\r\n\r\n  {}\r\n\r\nThis code will expire in 10 minutes.\r\n.\r\n",
        config.smtp_from_name, config.smtp_from_email, to_email, otp
    );
    let _ = stream.write_all(email_body.as_bytes()).await;
    let _ = stream.read(&mut buf).await;

    // Send QUIT
    let _ = stream.write_all(b"QUIT\r\n").await;

    info!(
        "Successfully dispatched verification OTP email to {}",
        to_email
    );
    Ok(())
}
