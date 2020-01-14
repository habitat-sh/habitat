using System;
using System.Diagnostics;
using System.IO;

public class SupervisorRunner {
    private String logPath = null;

    public Process Run(string logPath, string[] args) {
        this.logPath = logPath;
        var proc = new Process();
        proc.StartInfo.UseShellExecute = false;
        proc.StartInfo.CreateNoWindow = true;
        proc.StartInfo.RedirectStandardOutput = true;
        proc.StartInfo.RedirectStandardError = true;
        proc.StartInfo.FileName = "hab";
        proc.StartInfo.Arguments = "sup run --no-color " + String.Join(" ", args);
        proc.StartInfo.EnvironmentVariables["HAB_NOCOLORING"] = "1";
        proc.OutputDataReceived += new DataReceivedEventHandler(SupOutputHandler);
        proc.ErrorDataReceived += new DataReceivedEventHandler(SupOutputHandler);
        proc.Start();
        proc.BeginErrorReadLine();
        proc.BeginOutputReadLine();
        return proc;
    }

    private void SupOutputHandler(object sender, DataReceivedEventArgs e) {
        if (!String.IsNullOrEmpty(e.Data)) {
            using (FileStream fs = new FileStream(logPath, FileMode.Append, FileAccess.Write, FileShare.ReadWrite)) {
                using (StreamWriter sw = new StreamWriter(fs, System.Text.Encoding.UTF8)) {
                    sw.WriteLine(e.Data);
                }
            }
        }
    }
}
