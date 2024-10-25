#include "environment.iss"

[Setup]
AppName=Create Py App
AppVersion=1.0.0
DefaultDirName={autopf}\CreatePyApp
DefaultGroupName=CreatePyApp
OutputDir=.
OutputBaseFilename=CreatePyAppInstaller
Compression=lzma
SolidCompression=yes
ChangesEnvironment=true
PrivilegesRequired=admin
WizardStyle=modern


[Tasks]
Name: envPath; Description: "Add to PATH variable"



[Files]
Source: "..\..\target\x86_64-pc-windows-gnu\release\create-py-app.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Create Py App"; Filename: "{app}\create-py-app.exe"

[Run]
; This adds the install directory to the system PATH and logs output
Filename: "cmd"; Parameters: "/c setx /M PATH ""%PATH%;{app}"" > {app}\pathlog.txt"; Flags: runhidden

[UninstallRun]
; This removes the install directory from the system PATH when uninstalled
Filename: "cmd"; Parameters: "/c setx /M PATH ""%PATH:{app};=%"""; Flags: runhidden



[Code]
procedure CurStepChanged(CurStep: TSetupStep);
begin
    if (CurStep = ssPostInstall) and WizardisTaskSelected('envPath')
    then EnvAddPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
    if CurUninstallStep = usPostUninstall
    then EnvRemovePath(ExpandConstant('{app}'));
end;
