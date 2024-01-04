using CommunityToolkit.Mvvm.Input;
using Rake.Gui.Model.Commands;

namespace Rake.Gui.Win.ViewModel;

public class EditMenuItem
{
    public IRelayCommand UndoCommand { get; } = new RelayCommand(EditorCommands.Undo);
    public IRelayCommand RedoCommand { get; } = new RelayCommand(EditorCommands.Redo);
}