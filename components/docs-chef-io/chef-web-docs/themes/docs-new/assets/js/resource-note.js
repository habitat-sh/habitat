function resource_note_function(id) {
  var x = document.getElementById(id);
  if (x.className.indexOf("note-show") == -1) {
    x.className += " note-show";
  } else { 
    x.className = x.className.replace(" note-show", "");
  }
}