using Microsoft.Win32;

namespace Rake.Gui.Model.Native.Windows;

public class SaveDialog : ISaveDialog
{
    public string? FileName { get; private set; }
    public bool? Accepted { get; private set; }
    public string? Title { get; set; }
    public string DefaultFileName { get; set; } = string.Empty;
    public string DefaultExtension { get; set; } = string.Empty;
    public string? DefaultDirectory { get; set; }
    public IEnumerable<(string Description, string Glob)> Filters { get; set; } = Enumerable.Empty<(string, string)>();

    private string NativeFilters => string.Join("|", Filters.Select(pair => $"{pair.Description}|{pair.Glob}"));
    
    private SaveFileDialog GetNativeSaveDialog()
    {
        SaveFileDialog dialog = new();
        
        if (Title is not null)
            dialog.Title = Title;
        
        if (DefaultDirectory is not null)
            dialog.DefaultDirectory = DefaultDirectory;
        
        dialog.FileName = DefaultFileName;
        dialog.DefaultExt = DefaultExtension;
        dialog.Filter = NativeFilters;
        dialog.AddExtension = true;
        dialog.OverwritePrompt = true;

        return dialog;
    }

    public void Launch()
    {
        var dialog = GetNativeSaveDialog();
        Accepted = dialog.ShowDialog();
        FileName = dialog.FileName;
    }
}