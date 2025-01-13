# -*- coding: utf-8 -*-
#
# This file is part of EventGhost.
# Copyright © 2005-2019 EventGhost Project <http://www.eventghost.net/>
#
# EventGhost is free software: you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free
# Software Foundation, either version 2 of the License, or (at your option)
# any later version.
#
# EventGhost is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
# more details.
#
# You should have received a copy of the GNU General Public License along
# with EventGhost. If not, see <http://www.gnu.org/licenses/>.

import eg

eg.RegisterPlugin(
    name="Google Chrome",
    guid="{2CD4676D-6C65-499F-B538-3B59878F76A9}",
    author="Medy",
    version="0.0.1b",
    kind="other",
    description="Connecting EG with Chrome Browser and vise versa for native control.",
    canMultiLoad=False,
    createMacrosOnAdd=False
)

import json
import threading  # NOQA
import wx  # NOQA
import websocket_server  # NOQA



class Text(eg.TranslatableStrings):
    ip_label = 'IP Address:'
    port_label = 'Port:'

    class SendMessage:
        name = 'Send Message'
        description = 'Send a message to the browser'
        label = 'Message:'


class Server(websocket_server.WebsocketServer, object):
    request_queue_size = 1

    def __init__(self, plugin):
        self._thread = None
        self.plugin = plugin
        self._connected_client = None

    def start(self, host, port):
        websocket_server.WebsocketServer.__init__(self, port, host)
        self._thread = threading.Thread(target=self.run_forever)
        self._thread.start()

    def run_forever(self):
        websocket_server.WebsocketServer.run_forever(self)

    def stop(self):
        self.shutdown()
        self._thread = None
        self._connected_client = None

    def new_client(self, client, _):
        self.plugin.TriggerEvent('Browser.Connected')
        self._connected_client = client

    def client_left(self, client, _):
        self.plugin.TriggerEvent('Browser.Disconnected')
        self._connected_client = None

    def message_received(self, _, __, message):
        # 1. PARSING THE STRING to JSON format

        #self.plugin.TriggerEvent('IncomingMessage', message)
        a = IncomingMessageHandler()
        method = a.command_received
        method(self.plugin , message) # SENDING message to IncomingMessageHandler() Function.

    def send(self, message):
        if self._connected_client is not None:
            self._connected_client['handler'].send_message(message)


def _h_sizer(label, ctrl):
    sizer = wx.BoxSizer(wx.HORIZONTAL)
    sizer.Add(label, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)
    sizer.Add(ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)
    return sizer


class GoogleChrome(eg.PluginBase):

    text = Text

    def __init__(self):
        super(GoogleChrome, self).__init__()
        self.server = Server(self)
        #self.AddAction(SendMessage)
        self.AddAction(NewTab)
        self.AddAction(MoveTab)
        self.AddAction(RemoveTab)
        self.AddAction(UpdateTab)
        self.AddAction(ReloadTab)
        self.AddAction(QueryTabByIndex)
        self.AddAction(QueryActiveTab)

    def __start__(self): # ip , port add back for configure
        ip = 'localhost'
        port = 8000
        self.server.start(ip, port)

    def __stop__(self):
        self.server.stop()

    # def Configure(self, ip='', port=8000):
    #     panel = eg.ConfigPanel()
    #     ip_label = panel.StaticText(self.text.ip_label)
    #     port_label = panel.StaticText(self.text.port_label)
    #
    #     ip_ctrl = panel.TextCtrl(ip)
    #     port_ctrl = panel.SpinIntCtrl(port, min=1, max=65535)
    #
    #     eg.EqualizeWidths((ip_label, port_label))
    #
    #     panel.sizer.Add(_h_sizer(ip_label, ip_ctrl))
    #     panel.sizer.Add(_h_sizer(port_label, port_ctrl))
    #
    #     while panel.Affirmed():
    #         panel.SetResult(
    #             ip_ctrl.GetValue(),
    #             port_ctrl.GetValue()
    #         )


class SendMessage(eg.ActionBase):

    def __call__(self, message):
        self.plugin.server.send(message)

    def Configure(self, message=''):
        panel = eg.ConfigPanel()

        label = panel.StaticText(self.text.label) # 1. Select widget
        ctrl = wx.TextCtrl(panel, -1, message, style=wx.TE_MULTILINE) # 1. Select widget

        panel.sizer.Add(_h_sizer(label, ctrl)) # sizing both

        while panel.Affirmed():
            panel.SetResult(ctrl.GetValue())

# single Command
class NewTab(eg.ActionBase):

    name = "Create New Tab"
    description = "Create a new Tab and select properties & position"

    def __call__(self, value1, value2, value3, value5, value6):
        value1 = eg.ParseString(value1)
        message = {'command': 'NewTab', 'parameters': {'url': value1, 'active': value2, 'pinned': value3, 'target': value5, 'index': value6}}
        y = json.dumps(message)
        self.plugin.server.send(y)

    # https://effbot.org/zone/default-values.htm  ... default value so eg.payload works in input fields
    def Configure(self, value1='', value2=False, value3=False,value5=0,value6=''):

        panel = eg.ConfigPanel() # 1

        static_ctrl = wx.StaticText(panel, -1, 'Put URL here')  # 2   URL

        text_ctrl = wx.TextCtrl(panel, -1)  # 2
        text_ctrl.SetValue(str(value1))
        static2_ctrl = wx.StaticText(panel, -1, 'Set properties:')  # 2

        #Wx.CheckBox(parent, id, label, pos, size, style)
        checkbox = wx.CheckBox(panel, -1, 'active')  # 2  ACTIVE
        checkbox.SetValue(value2)
        checkbox2 = wx.CheckBox(panel, -1, 'pinned')  # 2
        checkbox2.SetValue(value3)

        lblList = ['Last Position', 'Index']
        #Wx.RadioBox(parent, id, label, pos, size, choices[], initialdimensions, style)
        radio = wx.RadioBox(panel,-1, label='Create at', pos=(80, 10), choices=lblList,
                            majorDimension=1, style=wx.RA_SPECIFY_ROWS)
        radio.SetSelection(value5)

        #SpinCtrl(parent, id=ID_ANY, value="", pos=DefaultPosition,size=DefaultSize, style=SP_ARROW_KEYS, min=0, max=100, initial=0,name="wxSpinCtrl")
        spin_ctrl = wx.SpinCtrl(panel,-1, str(value6), pos=(55, 90), size=(60, -1), min=0, max=100 ,  initial=0)  # 2
        line = wx.StaticLine(panel)


        sizer = wx.BoxSizer(wx.VERTICAL)  # 3


        sizer.Add(static_ctrl, 0, wx.ALIGN_CENTER_HORIZONTAL | wx.TOP, 5)  # 4
        sizer.Add(text_ctrl, 0, wx.EXPAND | wx.ALIGN_CENTER_HORIZONTAL | wx.BOTTOM, border=20)  # 4
        sizer.Add(static2_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(checkbox, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(checkbox2, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(line, 0,
                  flag=wx.EXPAND | wx.BOTTOM | wx.TOP, border=20)
        sizer.Add(radio, 0, wx.LEFT, border=5)  # 4
        sizer.Add(spin_ctrl, 0, wx.LEFT, border=5)  # 4
        #sizer.Add(static3_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4

        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            panel.SetResult(text_ctrl.GetValue(),checkbox.GetValue(),checkbox2.GetValue(),radio.GetSelection(),spin_ctrl.GetValue())

# Command with user Input
class UpdateTab(eg.ActionBase):

    name = "Update Tab"
    description = "Visit a new URL on the active Tab"

    def __call__(self, value1, value2, value3, value4, value5, value6):
        value1 = eg.ParseString(value1)
        message = {'command': 'NewUrl', 'parameters': {'url': value1, 'active': value2, 'pinned': value3, 'muted': value4, 'target': value5, 'index': value6}}
        y = json.dumps(message)
        self.plugin.server.send(y)


    def Configure(self, value1='', value2=False, value3=False,value4=False,value5=0,value6=''):
        #1.define  panel .. eventghost uses  eg.ConfigPanel() ... normal python uses x.Panel(self)
        #2 Select your widgets
        #3 select sizer oriantaiion (HORIZONTAL / VERTICAL)
        #4 add widgets to sizer and cofigure with flags
        #5 add sizer to panel
        #6 catch values from input (when panel gets affirmed / user affirrm .. übernehemen)

        # the "message" variable will be whats used ... you can add multiple just do comma seperate
        # panel.SetResult(text_ctrl.GetValue()) ,  panel.SetResult(text_ctrl2GetValue())

        # It is needed to use panel.Affirmed() and
        # panel.SetResult(…) in a loop, because the user might also use the
        # Apply button and EventGhost needs to know the current settings from the panel
        # without dismissing it completely.
        # http://www.eventghost.net/docs/writing_plugins.html?highlight=export#making-a-plugin-configurable
        # read about flags to shape / position  widgets  https://wxpython.org/Phoenix/docs/html/sizers_overview.html#the-flags-and-border-parameters

        panel = eg.ConfigPanel() # 1

        static_ctrl = wx.StaticText(panel, -1, 'Put URL here')  # 2
        #(self, parent, id=ID_ANY, value=””, pos=DefaultPosition, size=DefaultSize, style=0, validator=DefaultValidator, name=TextCtrlNameStr)
        text_ctrl = wx.TextCtrl(panel, -1)  # 2
        text_ctrl.SetValue(str(value1))
        static2_ctrl = wx.StaticText(panel, -1, 'Set properties:')  # 2

        #Wx.CheckBox(parent, id, label, pos, size, style)
        checkbox = wx.CheckBox(panel, -1, 'active')  # 2
        checkbox.SetValue(value2)
        checkbox2 = wx.CheckBox(panel, -1, 'pinned')  # 2
        checkbox2.SetValue(value3)
        checkbox3 = wx.CheckBox(panel, -1, 'muted')  # 2
        checkbox3.SetValue(value4)

        lblList = ['Active Tab', 'Index']
        #Wx.RadioBox(parent, id, label, pos, size, choices[], initialdimensions, style)
        radio = wx.RadioBox(panel,-1, label='Select Target by', pos=(80, 10), choices=lblList,
                            majorDimension=1, style=wx.RA_SPECIFY_ROWS)
        radio.SetSelection(value5)

        #SpinCtrl(parent, id=ID_ANY, value="", pos=DefaultPosition,size=DefaultSize, style=SP_ARROW_KEYS, min=0, max=100, initial=0,name="wxSpinCtrl")
        spin_ctrl = wx.SpinCtrl(panel,-1, str(value6), pos=(55, 90), size=(60, -1), min=0, max=100 ,  initial=0)  # 2
        line = wx.StaticLine(panel)

        #static3_ctrl = wx.StaticText(panel, -1, '*Leave "X" if Active Tab is selected')  # 2

        sizer = wx.BoxSizer(wx.VERTICAL)  # 3


        sizer.Add(static_ctrl, 0, wx.ALIGN_CENTER_HORIZONTAL | wx.TOP, 5)  # 4
        sizer.Add(text_ctrl, 0, wx.EXPAND | wx.ALIGN_CENTER_HORIZONTAL | wx.BOTTOM, border=20)  # 4
        sizer.Add(static2_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(checkbox, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(checkbox2, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(checkbox3, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(line, 0,
                  flag=wx.EXPAND | wx.BOTTOM | wx.TOP, border=20)
        sizer.Add(radio, 0, wx.LEFT, border=5)  # 4
        sizer.Add(spin_ctrl, 0, wx.LEFT, border=5)  # 4
        #sizer.Add(static3_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4

        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            #print(text_ctrl.GetValue(),'URL')
            #print(checkbox.GetValue(), 'cb1')
            #print(checkbox2.GetValue(), 'cb2')
            #print(checkbox3.GetValue(), 'cb3')
            #print(radio.GetSelection(), 'radio')
            #print(spin_ctrl.GetValue(), 'spinner')

            panel.SetResult(text_ctrl.GetValue(),checkbox.GetValue(),checkbox2.GetValue(),checkbox3.GetValue(),radio.GetSelection(),spin_ctrl.GetValue())



# single Command
class ReloadTab(eg.ActionBase):

    name = "Reload Tab"
    description = "Reload the active tab  or choose the index of the tab you want to reload"

    def __call__(self, value5, value6, value7):
        message = {'command': 'ReloadTab', 'parameters': {'target': value5, 'index': value6,'bypasscache': value7}}
        y = json.dumps(message)
        self.plugin.server.send(y)


    def Configure(self,value5=0,value6='', value7=False):

        panel = eg.ConfigPanel() # 1



        lblList = ['Active Tab', 'Index']
        #Wx.RadioBox(parent, id, label, pos, size, choices[], initialdimensions, style)
        radio = wx.RadioBox(panel,-1, label='Reload at', pos=(80, 10), choices=lblList,
                            majorDimension=1, style=wx.RA_SPECIFY_ROWS)
        radio.SetSelection(value5)

        #SpinCtrl(parent, id=ID_ANY, value="", pos=DefaultPosition,size=DefaultSize, style=SP_ARROW_KEYS, min=0, max=100, initial=0,name="wxSpinCtrl")
        spin_ctrl = wx.SpinCtrl(panel,-1, str(value6), pos=(55, 90), size=(60, -1), min=0, max=100 ,  initial=0)  # 2
        checkbox = wx.CheckBox(panel, -1, 'bypasscache')  # 2
        checkbox.SetValue(value7)


        sizer = wx.BoxSizer(wx.VERTICAL)  # 3


        sizer.Add(radio, 0, wx.LEFT, border=5)  # 4
        sizer.Add(spin_ctrl, 0, wx.LEFT, border=5)  # 4
        sizer.Add(checkbox, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        #sizer.Add(static3_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4

        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            panel.SetResult(radio.GetSelection(),spin_ctrl.GetValue(),checkbox.GetValue())

# single Command
class MoveTab(eg.ActionBase):

    name = "Move Tab"
    description = "Move a single tab to a new position / index "

    def __call__(self, value5, value6, value7):
        message = {'command': 'MoveTab', 'parameters': {'target': value5, 'startindex': value6, 'endindex': value7}}
        y = json.dumps(message)
        self.plugin.server.send(y)


    def Configure(self,value5=0,value6='',value7=''):

        panel = eg.ConfigPanel() # 1



        lblList = ['Active Tab', 'Index']
        #Wx.RadioBox(parent, id, label, pos, size, choices[], initialdimensions, style)
        radio = wx.RadioBox(panel,-1, label='Move from', pos=(80, 10), choices=lblList,
                            majorDimension=1, style=wx.RA_SPECIFY_ROWS)
        radio.SetSelection(value5)

        #SpinCtrl(parent, id=ID_ANY, value="", pos=DefaultPosition,size=DefaultSize, style=SP_ARROW_KEYS, min=0, max=100, initial=0,name="wxSpinCtrl")
        spin_ctrl = wx.SpinCtrl(panel,-1, str(value6), pos=(55, 90), size=(60, -1), min=0, max=100 ,  initial=0)  # 2
        line = wx.StaticLine(panel)
        static3_ctrl = wx.StaticText(panel, -1, 'Move to Index:')  # 2
        spin2_ctrl = wx.SpinCtrl(panel, -1, str(value7), pos=(55, 90), size=(60, -1), min=0, max=100, initial=0)  # 2



        sizer = wx.BoxSizer(wx.VERTICAL)  # 3


        sizer.Add(radio, 0, wx.LEFT, border=5)  # 4
        sizer.Add(spin_ctrl, 0, wx.LEFT, border=5)  # 4
        sizer.Add(line, 0,
                  flag=wx.EXPAND | wx.BOTTOM | wx.TOP, border=20)
        sizer.Add(static3_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4
        sizer.Add(spin2_ctrl, 0, wx.LEFT, border=5)  # 4


        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            panel.SetResult(radio.GetSelection(),spin_ctrl.GetValue(), spin2_ctrl.GetValue())


# single Command
class RemoveTab(eg.ActionBase):

    name = "Remove Tab"
    description = "Remove Selected Tab  / Close Tab"

    def __call__(self, value5, value6):
        message = {'command': 'RemoveTab', 'parameters': {'target': value5, 'index': value6}}
        y = json.dumps(message)
        self.plugin.server.send(y)


    def Configure(self,value5=0,value6=''):

        panel = eg.ConfigPanel() # 1



        lblList = ['Active Tab', 'Index']
        #Wx.RadioBox(parent, id, label, pos, size, choices[], initialdimensions, style)
        radio = wx.RadioBox(panel,-1, label='Remove Tab at', pos=(80, 10), choices=lblList,
                            majorDimension=1, style=wx.RA_SPECIFY_ROWS)
        radio.SetSelection(value5)

        #SpinCtrl(parent, id=ID_ANY, value="", pos=DefaultPosition,size=DefaultSize, style=SP_ARROW_KEYS, min=0, max=100, initial=0,name="wxSpinCtrl")
        spin_ctrl = wx.SpinCtrl(panel,-1, str(value6), pos=(55, 90), size=(60, -1), min=0, max=100 ,  initial=0)  # 2


        sizer = wx.BoxSizer(wx.VERTICAL)  # 3


        sizer.Add(radio, 0, wx.LEFT, border=5)  # 4
        sizer.Add(spin_ctrl, 0, wx.LEFT, border=5)  # 4
        #sizer.Add(static3_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5)  # 4

        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            panel.SetResult(radio.GetSelection(),spin_ctrl.GetValue())

# single Command
class QueryActiveTab(eg.ActionBase):

    name = "Query Active Tab"
    description = "Recieve JSON properties from active Tab"

    def __call__(self):
        message = {'command': 'QueryActiveTab'}
        y = json.dumps(message)
        self.plugin.server.send(y)


# Command with multiple filter options and user input
# Selecting Tabs that match filter , or simply query all tabs
class QueryTab(eg.ActionBase):

    name = "Query Tab"
    description = "Recieve SON properties from active Tab"

    def __call__(self, message):
        message = {'command': 'QueryTab', 'url': message}
        y = json.dumps(message)
        self.plugin.server.send(y)

    def Configure(self, message=''):


        panel = eg.ConfigPanel() # 1

        static_ctrl = wx.StaticText(panel, -1, 'Put yor Url here')  # 2
        text_ctrl = wx.TextCtrl(panel, -1, message)  # 2

        sizer = wx.BoxSizer(wx.VERTICAL) # 3


        sizer.Add(static_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5) #4
        sizer.Add(text_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5) #4

        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            panel.SetResult(text_ctrl.GetValue())

############################################################################
# EXPANSION 1 - MOVING and QUERY TAB Position


class QueryTabByIndex(eg.ActionBase):

    name = "Query Tab By Index"
    description = "QueryTabByIndex"

    def __call__(self, message):
        message = {'command': 'QueryTabByIndex', 'data': message}
        y = json.dumps(message)
        self.plugin.server.send(y)

    def Configure(self, message=''):


        panel = eg.ConfigPanel() # 1

        sizer = wx.GridBagSizer(5, 5)

        st1 = wx.StaticText(panel, label='Select Index of Tab you want to query')
        sizer.Add(st1, pos=(0, 0), span=(1, 2), flag=wx.ALL, border=15)

        st2 = wx.StaticText(panel, label='Index')
        sizer.Add(st2, pos=(1, 0), flag=wx.ALL | wx.ALIGN_CENTER, border=15)

        sc = wx.SpinCtrl(panel, 0, str(message))
        sc.SetRange(0, 100)

        sizer.Add(sc, pos=(1, 1), flag=wx.ALIGN_CENTER)

        panel.sizer.Add(sizer) #5


        #6
        while panel.Affirmed():
            panel.SetResult(sc.GetValue())


# class MoveTab(eg.ActionBase):
#
#     name = "Move Tab"
#     description = "Use the id  value from JSON to move tab to a new Position"
#
#     def __call__(self, message):
#         message = {'command': 'MoveTabToPosition', 'url': message}
#         y = json.dumps(message)
#         self.plugin.server.send(y)
#
#     def Configure(self, message=''):
#
#
#         panel = eg.ConfigPanel() # 1
#
#         static_ctrl = wx.StaticText(panel, -1, 'Put yor Url here')  # 2
#         text_ctrl = wx.TextCtrl(panel, -1, message)  # 2
#
#         sizer = wx.BoxSizer(wx.VERTICAL) # 3
#
#
#         sizer.Add(static_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5) #4
#         sizer.Add(text_ctrl, 0, wx.ALIGN_CENTER_VERTICAL | wx.ALL, 5) #4
#
#         panel.sizer.Add(sizer) #5
#
#
#         #6
#         while panel.Affirmed():
#             panel.SetResult(text_ctrl.GetValue())


############################################################################
############################################################################
############################################################################

# INCOMING MESSAGE HANDLER FUNCTION
# > Creating Events with eg.triggervent based on incoming command.

class IncomingMessageHandler:
# 2 accessing the JSON "ommand:" property

    def command_received(self, plugin, message):
        # TriggerEvent(suffix, payload=None)
        # first parameter suffix second is payload
        # http://www.eventghost.net/docs/eg/eg.PluginBase.html?highlight=trigger#eg.PluginBase.TriggerEvent
        print(message)
        y = json.loads(message)

        if y['command'] == 'QueryActiveTab':
            plugin.TriggerEvent('QueryActiveTabInfo', message)
            plugin.TriggerEvent('QueryActiveTab', y['data']['url'])

        elif y['command'] == 'QueryTabByIndex':
            plugin.TriggerEvent('QueryTabByIndex', y['data']['index'])
            plugin.TriggerEvent('QueryTabByIndexInfo', message)

#######################################################################

        elif y['command'] == 'ActiveTab':
            plugin.TriggerEvent('ActiveTabUrl', y['data']['url'])
            plugin.TriggerEvent('ActiveTabInfo', message)

        elif y['command'] == 'TabUpdated':
            plugin.TriggerEvent('ActiveTabUrl', y['data']['url'])
            plugin.TriggerEvent('ActiveTabUrInfo', message)

        elif y['command'] == 'CreateNewTab':
            #plugin.TriggerEvent('ActiveTabUrl', y['data']['url'])
            plugin.TriggerEvent('CreateNewTab', message)

        elif y['command'] == 'MoveTab':
            #plugin.TriggerEvent('ActiveTabUrl', y['data']['url'])
            plugin.TriggerEvent('MoveTab', message)

        elif y['command'] == 'RemoveTab':
            #plugin.TriggerEvent('ActiveTabUrl', y['data']['url'])
            plugin.TriggerEvent('RemoveTab', message)
#######################################################################