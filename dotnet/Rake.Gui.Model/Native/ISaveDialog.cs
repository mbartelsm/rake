namespace Rake.Gui.Model.Native;

public interface ISaveDialog
{
    public string? FileName { get; }
    
    public bool? Accepted { get; }
    public string Title { set; }
    public string DefaultFileName { set; }
    public string DefaultExtension { set; }
    public string DefaultDirectory { set; }
    public IEnumerable<(string Description, string Glob)> Filters { set; }

    public void Launch();
}