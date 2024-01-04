namespace Rake.Gui.Model.Native;

public interface IOpenDialog
{
    public string? FileName { get; }
    
    public bool? Accepted { get; }
    public string Title { set; }
    public string DefaultDirectory { set; }
    public IEnumerable<(string Description, string Glob)> Filters { set; }
    public bool Multiselect { set; }

    public void Launch();
}