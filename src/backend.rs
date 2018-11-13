use crate::failure_ext::OptionExt;
use crate::session::SessionMan;
use failure::Fallible;
use std::path::PathBuf;

pub struct SessionManMPI {
    mgr: SessionMan,
    progname: String,
    progdir: PathBuf,
}

impl SessionManMPI {
    pub fn new(mgr: SessionMan, progname: String) -> Self {
        let home = dirs::home_dir().expect("Unable to get the home dir");
        let progdir = home.join("pub").join(&progname);
        SessionManMPI {
            mgr,
            progname,
            progdir,
        }
    }

    pub fn make(&self) -> Fallible<()> {
        let progdir = self.progdir.to_str().expect("progdir is invalid utf8");
        self.mgr.exec("make", &["-C", progdir])
    }

    pub fn run(&self, nproc: u32, args: &[&str]) -> Fallible<()> {
        let progpath = self
            .progdir
            .join(&self.progname)
            .to_str()
            .expect("progpath is invalid utf8");
        let npstr = nproc.to_string();
        let mpiargs = [&["-n", &npstr, &self.progname], args].concat();
        self.mgr.exec("mpirun", &mpiargs)
    }
}
