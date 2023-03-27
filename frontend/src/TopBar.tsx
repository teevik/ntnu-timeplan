import { AppBar, Toolbar, Typography, Button } from "@mui/material";

export function TopBar() {
  return (
    <AppBar position="static">
      <Toolbar>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          NTNU Timeplan
        </Typography>
        <Button
          color="inherit"
          href="https://github.com/teevik/ntnu-timeplan"
          target="_blank"
        >
          Github
        </Button>
        <Button
          color="inherit"
          href="https://ntnu-timeplan-api.fly.dev/"
          target="_blank"
        >
          API
        </Button>
      </Toolbar>
    </AppBar>
  );
}
