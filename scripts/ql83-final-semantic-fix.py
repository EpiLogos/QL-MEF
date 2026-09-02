from pathlib import Path

path = Path('crates/ql-mef/src/vak_oi.rs')
source = path.read_text()

wrong = '''        if self.native_handler_ref.trim().is_empty() || self.native_result_lineage.trim().is_empty() {
            return Err(VakOiError::Missing("native Action handler/result lineage"));
        }
'''
if source.count(wrong) != 1:
    raise SystemExit(f'unexpected misplaced Action lineage validation count: {source.count(wrong)}')
source = source.replace(wrong, '', 1)

impl_marker = 'impl VakActionProfileV1 {'
impl_index = source.find(impl_marker)
if impl_index < 0:
    raise SystemExit('VakActionProfileV1 impl missing')
head = source[:impl_index]
tail = source[impl_index:]
owner_check = '''        if self.native_owner.trim().is_empty() {
            return Err(VakOiError::Missing("native_owner"));
        }
'''
if tail.count(owner_check) != 1:
    raise SystemExit(f'unexpected Action-profile native-owner validation count: {tail.count(owner_check)}')
lineage_check = owner_check + '''        if self.native_handler_ref.trim().is_empty() || self.native_result_lineage.trim().is_empty() {
            return Err(VakOiError::Missing("native Action handler/result lineage"));
        }
'''
tail = tail.replace(owner_check, lineage_check, 1)
path.write_text(head + tail)
