use crate::dispatch::{get_ver_from_env, names::CORE};
use anyhow::Result;
use const_format::formatcp;
use xshell::{cmd, write_file};

pub struct Binary<'s> {
    pub artifact: &'s str,
    pub platform: &'s str,
}

impl<'s> Binary<'s> {
    pub fn asset(&self) -> String {
        format!("{}-{}", self.artifact, self.platform)
    }

    pub fn archive(&self) -> String {
        self.asset() + ".tar.gz"
    }
}

pub enum BinaryBuilder<'s> {
    Native(Binary<'s>),

    /// A [`Binary`] generated by cross compilation.
    Cross {
        bin: Binary<'s>,
        rust_target: &'s str,
    },
}

impl<'s> BinaryBuilder<'s> {
    pub fn bin(&self) -> &Binary {
        match self {
            BinaryBuilder::Native(bin) => bin,
            BinaryBuilder::Cross { bin, .. } => bin,
        }
    }

    pub fn build(&self) -> Result<()> {
        println!(":: Building the binary in `release` mode...");
        match self {
            BinaryBuilder::Native(_) => {
                cmd!("cargo build --verbose --bin {CORE} --release --locked").run()?;
            }
            BinaryBuilder::Cross { rust_target, .. } => {
                cmd!("rustup target add {rust_target}").run()?;
                cmd!(
                    "cargo build --verbose --bin {CORE} --release --locked --target={rust_target}"
                )
                .run()?;
            }
        }
        Ok(())
    }

    pub fn bin_dir(&self) -> String {
        if let BinaryBuilder::Cross { rust_target, .. } = self {
            format!("./target/{}/release/", rust_target)
        } else {
            "./target/release/".to_owned()
        }
    }

    pub fn zip(&self) -> Result<()> {
        println!(":: Zipping the binary...");
        let bin = self.bin();
        let asset = &bin.asset();
        let artifact = &bin.artifact;
        let bin_dir = self.bin_dir();
        cmd!("tar czvf {asset}.tar.gz -C {bin_dir} {artifact}").run()?;

        println!(":: Generating sha256...");
        let shasum = cmd!("openssl dgst -r -sha256 {asset}.tar.gz").read()?;
        write_file(format!("{}.tar.gz.sha256", asset), shasum)?;
        Ok(())
    }

    pub fn upload(&self) -> Result<()> {
        println!(":: Uploading binary and sha256...");
        let tag = get_ver_from_env()?;
        let asset = self.bin().asset();
        cmd!("gh release upload {tag} {asset}.tar.gz {asset}.tar.gz.sha256").run()?;
        Ok(())
    }
}

pub const WIN_X64: Binary = Binary {
    artifact: formatcp!("{}.exe", CORE),
    platform: "windows-amd64",
};

pub const MAC_X64: Binary = Binary {
    artifact: CORE,
    platform: "macos-amd64",
};

pub const MAC_ARM: Binary = Binary {
    artifact: CORE,
    platform: "macos-aarch64",
};

pub const MAC_UNIV: Binary = Binary {
    artifact: CORE,
    platform: "macos-universal",
};

pub const LINUX_X64: Binary = Binary {
    artifact: CORE,
    platform: "linux-amd64",
};