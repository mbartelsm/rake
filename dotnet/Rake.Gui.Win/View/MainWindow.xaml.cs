using System.Windows;
using Rake.Gui.Win.ViewModel.Utilities;

namespace Rake.Gui.Win.View
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            ((IRequestClose)DataContext).RequestClose += (_, _) => Close();
            ((IRequestClose)TopMenu.File.DataContext).RequestClose += (_, _) => Close();
        }
    }
}