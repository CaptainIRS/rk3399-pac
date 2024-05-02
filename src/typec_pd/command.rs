#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `Command` writer - 0001 0001b Wake Interface (no action \n\nis \n\ntaken other than to wake the interface). \n\n0010 0010b DisableVbusDetect. Disable Vbus \n\npresent detection. The TCPC shall ignore this \n\ncommand \n\nand \n\nassert \n\nthe \n\nFAULT_STATUS.InterfaceError if it has sourcing \n\nor sinking power over Vbus enabled. \n\n0011 0011b EnableVbusDetect. Enable Vbus \n\npresent detection. \n\n0100 0100b DisableSinkVbus. Disable sinking \n\npower over Vbus. This COMMAND does not \n\ndisable \n\nPOWER_STATUS.VBUSPresent \n\ndetection. \n\nTheTCPC \n\nshall \n\nclear \n\nFAULT_STATUS.InternalorExternalOCP \n\nandFAULT_STATUS.InternalorExternalOVP. \n\n0101 0101b SinkVbus. Enable sinking power \n\nover Vbus and enable Vbus present detection. \n\nThe TCPC shall \n\nignore \n\nthis command and \n\nassert the FAULT_STATUS.InterfaceError if it \n\nhas sourcing power over Vbus enabled. \n\n0110 \n\n0110b DisableSourceVbus. Disable \n\nsourcing power over Vbus. The TCPC shall \n\nstop reporting FAULT_STATUS. Internal or \n\nExternal OCP or OVP Faults. This COMMAND \n\ndoes \n\nnot \n\ndisable \n\nPOWER_STATUS.VBUSPresentdetection. \n\n0111 0111b SourceVbusDefaultVoltage. Enable \n\nsourcing vSafe5V over Vbus and enable Vbus \n\npresent detection. Source shall transition to \n\nvSafe5V if at a high voltage. The TCPC shall \n\nignore \n\nthis \n\ncommand \n\nand \n\nassert \n\nthe \n\nFAULT_STATUS.InterfaceError if it has sinking\n\npower over Vbus enabled. \n\n1000 1000b SourceVbusHighVoltage. Execute \n\nsourcing high voltage over Vbus. The TCPC shall \n\nignore \n\nthis \n\ncommand \n\nand \n\nassert \n\nthe \n\nFAULT_STATUS.InterfaceError if it is currently \n\nsinking voltage from Vbus or does not have \n\nability to source voltages higher than vSafe5V. \n\nThe TCPC shall ignore this command and assert \n\nthe FAULT_STATUS.InterfaceError if not already \n\nsourcing vSafe5V. The actual voltage to be \n\nsourced may be set in a vendor defined manner. \n\nThe TCPM may need to send vendor defined \n\ncommands before sending this COMMAND. \n\n1001 \n\n1001b \n\nLook4Connection. Start DRP \n\nToggling if ROLE_CONTROL.DRP=1b.If \n\nROLE_CONTROL.CC1/CC2=01b start with Rp, if \n\nROLE_CONTROL.CC1/CC2=10b start with Rd. \n\nIf ROLE_CONTROL.CC1/CC2 are not 01b or \n\n10b, then do not start toggling. The TCPM shall \n\nissue COMMAND.Look4Connection \n\nto enable \n\nthe TCPCto restart Connection Detection \n\nin \n\ncases where the ROLE_CONTROL contents will \n\nnot change. An example of this is when a \n\npotential connection as a Source occurred but \n\nwas further debounced by the TCPM to find the \n\nSink disconnected. In this case a Source Only \n\nor DRP should go back to its Unattached.Src \n\nstate. This would result \n\nin ROLE_CONTROL \n\nstaying the same. TCPC to \n\nMAINTAIN_STATE in Figure4-11 \n\n1010 \n\n1010b \n\nRxOneMore. \n\nConfigure \n\nthe \n\nreceiver \n\nto \n\nautomatically \n\nclear \n\nthe \n\nRECEIVE_DETECT register after sending the \n\nnext GoodCRC. This \n\nis used \n\nto shutdown \n\nreception \n\nof \n\npackets \n\nat \n\na \n\nknown \n\npoint \n\nregardless of packet separation or the depth \n\nof the receive FIFO in the TCPC. \n\n1100 \n\n1100b \n\nReserved. \n\nNo \n\nAction \n\n1101 \n\n1101b \n\nReserved. \n\nNo \n\nAction \n\n1110 \n\n1110b \n\nReserved. \n\nNoAction \n\n1111 0000b Enable 1st VBUS source (sets bit \n\n\\[0\\]
of vbus_source_en output when VBUS \n\nsourcing is activated and clear all others).\n\n1111 0001b Enable 2nd VBUS source (sets bit \n\n\\[1\\]
of vbus_source_en output when VBUS \n\nsourcing is activated and clear all others). \n\n1111 0010b Enable 3rd VBUS source (sets bit \n\n\\[2\\]
of vbus_source_en output when vbus \n\nsourcing is activated and clear all others). \n\n1111 0011b Enable 4th VBUS source (sets bit \n\n\\[3\\]
of vbus_source_en output when VBUS \n\nsourcing is activated and clear all others). \n\n1111 1111b Reserved. NoAction"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore \n\nreadvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and \n\nignore \n\nreadvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Not used. Always write as 0 and \n\nignore \n\nreadvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0001 0001b Wake Interface (no action \n\nis \n\ntaken other than to wake the interface). \n\n0010 0010b DisableVbusDetect. Disable Vbus \n\npresent detection. The TCPC shall ignore this \n\ncommand \n\nand \n\nassert \n\nthe \n\nFAULT_STATUS.InterfaceError if it has sourcing \n\nor sinking power over Vbus enabled. \n\n0011 0011b EnableVbusDetect. Enable Vbus \n\npresent detection. \n\n0100 0100b DisableSinkVbus. Disable sinking \n\npower over Vbus. This COMMAND does not \n\ndisable \n\nPOWER_STATUS.VBUSPresent \n\ndetection. \n\nTheTCPC \n\nshall \n\nclear \n\nFAULT_STATUS.InternalorExternalOCP \n\nandFAULT_STATUS.InternalorExternalOVP. \n\n0101 0101b SinkVbus. Enable sinking power \n\nover Vbus and enable Vbus present detection. \n\nThe TCPC shall \n\nignore \n\nthis command and \n\nassert the FAULT_STATUS.InterfaceError if it \n\nhas sourcing power over Vbus enabled. \n\n0110 \n\n0110b DisableSourceVbus. Disable \n\nsourcing power over Vbus. The TCPC shall \n\nstop reporting FAULT_STATUS. Internal or \n\nExternal OCP or OVP Faults. This COMMAND \n\ndoes \n\nnot \n\ndisable \n\nPOWER_STATUS.VBUSPresentdetection. \n\n0111 0111b SourceVbusDefaultVoltage. Enable \n\nsourcing vSafe5V over Vbus and enable Vbus \n\npresent detection. Source shall transition to \n\nvSafe5V if at a high voltage. The TCPC shall \n\nignore \n\nthis \n\ncommand \n\nand \n\nassert \n\nthe \n\nFAULT_STATUS.InterfaceError if it has sinking\n\npower over Vbus enabled. \n\n1000 1000b SourceVbusHighVoltage. Execute \n\nsourcing high voltage over Vbus. The TCPC shall \n\nignore \n\nthis \n\ncommand \n\nand \n\nassert \n\nthe \n\nFAULT_STATUS.InterfaceError if it is currently \n\nsinking voltage from Vbus or does not have \n\nability to source voltages higher than vSafe5V. \n\nThe TCPC shall ignore this command and assert \n\nthe FAULT_STATUS.InterfaceError if not already \n\nsourcing vSafe5V. The actual voltage to be \n\nsourced may be set in a vendor defined manner. \n\nThe TCPM may need to send vendor defined \n\ncommands before sending this COMMAND. \n\n1001 \n\n1001b \n\nLook4Connection. Start DRP \n\nToggling if ROLE_CONTROL.DRP=1b.If \n\nROLE_CONTROL.CC1/CC2=01b start with Rp, if \n\nROLE_CONTROL.CC1/CC2=10b start with Rd. \n\nIf ROLE_CONTROL.CC1/CC2 are not 01b or \n\n10b, then do not start toggling. The TCPM shall \n\nissue COMMAND.Look4Connection \n\nto enable \n\nthe TCPCto restart Connection Detection \n\nin \n\ncases where the ROLE_CONTROL contents will \n\nnot change. An example of this is when a \n\npotential connection as a Source occurred but \n\nwas further debounced by the TCPM to find the \n\nSink disconnected. In this case a Source Only \n\nor DRP should go back to its Unattached.Src \n\nstate. This would result \n\nin ROLE_CONTROL \n\nstaying the same. TCPC to \n\nMAINTAIN_STATE in Figure4-11 \n\n1010 \n\n1010b \n\nRxOneMore. \n\nConfigure \n\nthe \n\nreceiver \n\nto \n\nautomatically \n\nclear \n\nthe \n\nRECEIVE_DETECT register after sending the \n\nnext GoodCRC. This \n\nis used \n\nto shutdown \n\nreception \n\nof \n\npackets \n\nat \n\na \n\nknown \n\npoint \n\nregardless of packet separation or the depth \n\nof the receive FIFO in the TCPC. \n\n1100 \n\n1100b \n\nReserved. \n\nNo \n\nAction \n\n1101 \n\n1101b \n\nReserved. \n\nNo \n\nAction \n\n1110 \n\n1110b \n\nReserved. \n\nNoAction \n\n1111 0000b Enable 1st VBUS source (sets bit \n\n\\[0\\]
of vbus_source_en output when VBUS \n\nsourcing is activated and clear all others).\n\n1111 0001b Enable 2nd VBUS source (sets bit \n\n\\[1\\]
of vbus_source_en output when VBUS \n\nsourcing is activated and clear all others). \n\n1111 0010b Enable 3rd VBUS source (sets bit \n\n\\[2\\]
of vbus_source_en output when vbus \n\nsourcing is activated and clear all others). \n\n1111 0011b Enable 4th VBUS source (sets bit \n\n\\[3\\]
of vbus_source_en output when VBUS \n\nsourcing is activated and clear all others). \n\n1111 1111b Reserved. NoAction"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<CommandSpec> {
        CommandW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and \n\nignore \n\nreadvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<CommandSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {
    const RESET_VALUE: u32 = 0;
}
