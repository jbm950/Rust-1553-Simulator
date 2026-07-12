Output assets/tui-demo.gif
Set FontSize 20
Set Width 1200
Set Height 600

Hide
Type "clear && ./run_sim.sh"
Enter
Sleep 4s
Show

Sleep 2s

# Switch to "Charging" mode
Type "j"
Sleep 400ms
Enter
Sleep 2s

# Inject "Overtemp" fault
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Enter
Sleep 2s

# Clear fault
Type "k"
Sleep 400ms
Enter

# Let the viewer observe the cleared fault before the loop restarts
Sleep 3s
