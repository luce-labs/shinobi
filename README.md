# Shinobi

  <img src="./忍.png" width="150" align="left" style="margin-right: 10px;" />
<p align="left">
  
#### How it works(First Design)
A project owner creates a project data on the server, project data includes secrets. The secret is encrypted and stored on a database. To share secrets with others the project owner requests to generate a qrcode, which contains project information and a keys token to decrypt the encrypted keys stored on the database. With the cli tool other users can submit the qrcode for decryption. When the qrcode is decrypted you get the keystoken , which is then passed into a secrets_server(daemon), the secrets server uses the token and an authorization header to request for the secrets from the server, the server checks and if that user is allowed to have the keys, it uses the token to get the secrets to the secrets_server(daemon), this communication is also end to end encrypted. When the secrets server gets the secrets, it stores them in a secure memory block which is on a different processor. So to get the secrets you need inter processor communication and that’s what the library does. The library requests for the secrets (end to end encrypted) and the secrets_server checks if the library is authorized, if it is, it releases the secrets in a custom data type that will only print([PROTECTED]) when you try to view it as in print it or return it to stdout or stderr but will get you the value when you try to use it programmertically, you can perform operations on it but you can’t view it.
</p>

<br clear="left" />

##### Design

<p align="center">
  <img src="./shinobi_architecture.png" />
</p>

Note: this tool is not yet safe enough and work is in progress, it will go through a lot of phases to make sure it's safe enough


