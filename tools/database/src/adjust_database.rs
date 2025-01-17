use std::path::Path;
use unc_store::metadata::DbKind;
use unc_store::NodeStorage;

/// This can potentially support db specified not in config, but in command line.
/// `ChangeRelative { path: Path, archive: bool }`
/// But it is a pain to implement, because of all the current storage possibilities.
/// So, I'll leave it as a TODO(posvyatokum): implement relative path DbSelector.
/// This can be useful workaround for config modification.
#[derive(clap::Subcommand)]
enum DbSelector {
    ChangeHot,
    ChangeCold,
}

#[derive(clap::Args)]
pub(crate) struct ChangeDbKindCommand {
    /// Desired DbKind.
    #[clap(long)]
    new_kind: DbKind,
    /// Which db to change.
    #[clap(subcommand)]
    db_selector: DbSelector,
}

impl ChangeDbKindCommand {
    pub(crate) fn run(&self, home_dir: &Path) -> anyhow::Result<()> {
        let unc_config = framework::config::load_config(
            &home_dir,
            unc_chain_configs::GenesisValidationMode::UnsafeFast,
        )?;
        let opener = NodeStorage::opener(
            home_dir,
            unc_config.config.archive,
            &unc_config.config.store,
            unc_config.config.cold_store.as_ref(),
        );

        let storage = opener.open()?;
        let store = match self.db_selector {
            DbSelector::ChangeHot => storage.get_hot_store(),
            DbSelector::ChangeCold => {
                storage.get_cold_store().ok_or(anyhow::anyhow!("No cold store"))?
            }
        };
        Ok(store.set_db_kind(self.new_kind)?)
    }
}
