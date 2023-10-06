# ?????????????

......

## Running with Docker

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

## License

Licensed under either of the following:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
