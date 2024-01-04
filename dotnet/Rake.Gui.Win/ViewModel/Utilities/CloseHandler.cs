using Rake.Gui.Model.Commands;

namespace Rake.Gui.Win.ViewModel.Utilities;

public class CloseHandler(Func<EventHandler?> getEvent, object? sender)
{
    public void Close()
    {
        GlobalCommands.Close();
        getEvent()?.Invoke(sender, EventArgs.Empty);
    }
}