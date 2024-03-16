fn predicate(line){
  let name = line.cmd_name;
  name!=()&&name=="python3"||name=="python"
}
fn completions(line){
  if is_short_flag(line){
    return default_format(["-h", "-V", "-c", "-m", "-i", "-q", "-S", "-u"]);

  }

  filename_completions(line).filter(|f|f.completion.ends_with(".py"))

}
