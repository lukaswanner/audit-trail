-- Add migration script here
-- add single project for local testing
INSERT INTO account(email, password) 
VALUES ('lukas.wanner@google.com', '$argon2id$v=19$m=19456,t=2,p=1$qfRVJdjcdbp8TSWXLUl6Bw$WokDWXNfsHzFbFXkzRRGpbGHwsQ3jSyev4EdTWK/Xyk');
