# ?????????????

.........

.... allow people to upload files to my file server using only a web browser (i.e. so users don't have to install a SFTP client or any other software).

Each file will be placed in it's own subdirectory with the date & time of upload as the directory name, so you can easily find your files. The idea is that after someone has uploaded a file, you would go grab it and move it somewhere else, so nothing is left in the upload directory long term.

## Running with Docker or Podman

...... forward port 8000 .... map /opt/file_upload/data mapped to wherever you want the files to be stored

## TODO

* [ ] Documentation
* [ ] Docker image
    * [x] Get it running
    * [ ] Usage instructions
    * [ ] Get it working with a smaller image like alpine
    * [x] Prevent "invalid cross-device link" error:
        * <https://github.com/SergioBenitez/Rocket/issues/1600>
        * <https://github.com/ShaddyDC/track-wear-backend/commit/e85bf54c0688900ff1f7052719f9676835402475>
    * [ ] Get it to work without the`set_permissions` call
        * This might be useful: <https://stackoverflow.com/questions/26500270/understanding-user-file-ownership-in-docker-how-to-avoid-changing-permissions-o/26514736#26514736>
* [ ] Improve `index.html`:
    * [x] Make it pretty
    * [ ] Allow files to be dragged and dropped
    * [x] Show acceptable file extensions
    * [ ] Check the file extension locally in the browser before upload so user doesn't have to wait for upload to complete on a file that will be rejected anyway
    * [x] Progress bar
* [ ] Authentication
* [x] Replace synchronous functions with async equivalents:
    * [x] `std::fs::create_dir_all`
    * [x] Creating and writing to `Original filename.txt`
* [ ] favicon
* [ ] Time zone - <https://stackoverflow.com/questions/57607381/how-do-i-change-timezone-in-a-docker-container>
* [ ] Sends a message to Slack when a file is uploaded

## License

Licensed under either of the following:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
