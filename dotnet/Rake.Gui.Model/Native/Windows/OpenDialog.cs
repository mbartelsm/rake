using Microsoft.Win32;

namespace Rake.Gui.Model.Native.Windows;

public class OpenDialog : IOpenDialog
{
    public string? FileName { get; private set; }
    public bool? Accepted { get; private set; }
    public string? Title { get; set; }
    public string? DefaultDirectory { get; set; }
    public IEnumerable<(string Description, string Glob)> Filters { get; set; } = Enumerable.Empty<(string, string)>();
    public bool Multiselect { get; set; } = false;

    private string NativeFilters => string.Join("|", Filters.Select(pair => $"{pair.Description}|{pair.Glob}"));
    
    private OpenFileDialog GetNativeSaveDialog()
    {
        OpenFileDialog dialog = new();
        
        if (Title is not null)
            dialog.Title = Title;
        
        if (DefaultDirectory is not null)
            dialog.DefaultDirectory = DefaultDirectory;
        
        dialog.Filter = NativeFilters;
        dialog.AddExtension = true;
        dialog.Multiselect = Multiselect;

        return dialog;
    }

    public void Launch()
    {
        var dialog = GetNativeSaveDialog();
        Accepted = dialog.ShowDialog();
        FileName = dialog.FileName;
    }
}