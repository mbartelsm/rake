namespace Rake.Gui.Model.Commands;

public static class EditorCommands
{
    public static void Undo()
    {
        Console.WriteLine($"Command: {nameof(Undo)}");
    }
    
    public static void Redo()
    {
        Console.WriteLine($"Command: {nameof(Redo)}");
    }
}