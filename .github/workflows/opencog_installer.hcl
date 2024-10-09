data "external" "opencog_installer" {
  program = ["./path/to/opencog_installer", "--add-repositories", "--install-dependencies"]
}

resource "null_resource" "install_opencog" {
  triggers = {
    always_run = "${timestamp()}"
  }

  provisioner "local-exec" {
    command = "./path/to/opencog_installer --add-repositories --install-dependencies"
  }
}
