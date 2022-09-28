use crate::auth::forms::Session;
use rocket::{
    data::ToByteUnit,
    form::Form,
    fs::{relative, NamedFile, TempFile},
    http::Status,
    request::FlashMessage,
    response::{Flash, Redirect},
    FromForm,
};
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};

//TODO: ADD VALIDATION TO LIMIT TO IMAGE FILES
#[derive(FromForm)]
#[allow(dead_code)]
pub struct Image<'v> {
    file: String,
    #[field(validate = len(..=10.mebibytes()))]
    image: TempFile<'v>,
}

#[post("/create", data = "<image>")]
pub async fn upload(
    mut image: Form<Image<'_>>,
    sess: Session,
) -> Result<Flash<Redirect>, (Status, Flash<Redirect>)> {
    let filename = image.file.clone();
    if sess.isadmin {
        let path = Path::new(relative!("media")).join(PathBuf::from(filename.clone()));
        if path.exists() {
            return Err((
                Status::UnprocessableEntity,
                Flash::error(
                    Redirect::to("/media/create"),
                    "A file by that name already exists",
                ),
            ));
        }
        println!("{:#?}", image.image.path());
        match image.image.copy_to(path).await {
            Ok(_) => Ok(Flash::success(
                Redirect::to("/media/create"),
                format!(
                    "File has been created, it can be accessed at /media/{}",
                    filename
                ),
            )),
            Err(e) => Err((
                Status::UnprocessableEntity,
                Flash::error(
                    Redirect::to("/media/create"),
                    format!("unable to process file:\n{e}"),
                ),
            )),
        }
    } else {
        Err((
            Status::Unauthorized,
            Flash::error(
                Redirect::to("/users/login"),
                "you must be logged in as an admin to view this page",
            ),
        ))
    }
}

#[get("/create")]
pub async fn image_form(
    sess: Session,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, (Status, Flash<Redirect>)> {
    if sess.isadmin {
        Ok(Template::render(
            "image",
            context! {title:"Upload a image",flash:flash},
        ))
    } else {
        Err((
            Status::Unauthorized,
            Flash::error(
                Redirect::to("/users/login"),
                "you must be logged in as an admin to view this page",
            ),
        ))
    }
}

#[get("/<asset>")]
pub async fn media(asset: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("media")).join(asset);
    if path.is_dir() {
        return None;
    }
    NamedFile::open(path).await.ok()
}
