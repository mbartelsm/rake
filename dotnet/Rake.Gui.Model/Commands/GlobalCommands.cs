using System.Windows;
using Rake.Gui.Model.Native;

namespace Rake.Gui.Model.Commands;

public static class GlobalCommands
{
    public static void SaveAs()
    {
        Console.WriteLine($"Command: {nameof(SaveAs)}");
        ISaveDialog dialog = new Native.Windows.SaveDialog();

        dialog.DefaultDirectory = Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments);
        dialog.DefaultFileName = "Untitled";
        dialog.DefaultExtension = ".txt";
        dialog.Filters = new[] { ("Plain text", "*.txt") };
        
        dialog.Launch();
        
        if (dialog.Accepted is true)
            Console.WriteLine($"   Saved '{dialog.FileName}'");
        else
            Console.WriteLine($"   Aborted: Save as file");
    }
    
    public static void Open()
    {
        Console.WriteLine($"Command: {nameof(Open)}");
        IOpenDialog dialog = new Native.Windows.OpenDialog();

        dialog.DefaultDirectory = Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments);
        dialog.Filters = new[] { ("Plain text", "*.txt") };
        
        dialog.Launch();
        
        if (dialog.Accepted is true)
            Console.WriteLine($"    Opened '{dialog.FileName}'");
        else
            Console.WriteLine($"    Aborted: Open file");
    }

    public static void Close()
    {
        Console.WriteLine($"Closing window");
    }
}