using System.Windows;
using CommunityToolkit.Mvvm.Input;
using Rake.Gui.Model.Commands;
using Rake.Gui.Win.ViewModel.Utilities;

namespace Rake.Gui.Win.ViewModel;

public class FileMenuItem : IRequestClose
{
    public IRelayCommand OpenCommand { get; } = new RelayCommand(GlobalCommands.Open);
    public IRelayCommand SaveAsCommand { get; } = new RelayCommand(GlobalCommands.SaveAs);
    
    public IRelayCommand CloseCommand => _closeCommand ??= new RelayCommand(CloseHandler.Close);
    private IRelayCommand? _closeCommand;

    public event EventHandler? RequestClose;

    private CloseHandler CloseHandler => _closeHandler ??= new CloseHandler(() => RequestClose, this);
    private CloseHandler? _closeHandler;
}