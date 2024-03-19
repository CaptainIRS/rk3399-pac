#[doc = "Register `USB3_DEP%sCMD` reader"]
pub type R = crate::R<Usb3DepcmdSpec>;
#[doc = "Register `USB3_DEP%sCMD` writer"]
pub type W = crate::W<Usb3DepcmdSpec>;
#[doc = "Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtyp {
    #[doc = "0: Reserved"]
    H00 = 0,
    #[doc = "1: Set Endpoint Configuration - -64 or 96-bit Parameter"]
    H01 = 1,
    #[doc = "2: Set Endpoint Transfer Resource Configuration - 32- bitparameter"]
    H02 = 2,
    #[doc = "3: Get Endpoint State - No Parameter Needed"]
    H03 = 3,
    #[doc = "4: Set Stall - No Parameter Needed"]
    H04 = 4,
    #[doc = "5: Clear Stall (see Set Stall) - No Parameter Needed"]
    H05 = 5,
    #[doc = "6: Start Transfer - 64-bit Parameter"]
    H06 = 6,
    #[doc = "7: Update Transfer - No Parameter Needed"]
    H07 = 7,
    #[doc = "8: End Transfer - No Parameter Needed"]
    H08 = 8,
    #[doc = "9: Start New Configuration - No Parameter Needed"]
    H09 = 9,
}
impl From<Cmdtyp> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtyp {
    type Ux = u8;
}
#[doc = "Field `CMDTYP` reader - Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform."]
pub type CmdtypR = crate::FieldReader<Cmdtyp>;
impl CmdtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdtyp> {
        match self.bits {
            0 => Some(Cmdtyp::H00),
            1 => Some(Cmdtyp::H01),
            2 => Some(Cmdtyp::H02),
            3 => Some(Cmdtyp::H03),
            4 => Some(Cmdtyp::H04),
            5 => Some(Cmdtyp::H05),
            6 => Some(Cmdtyp::H06),
            7 => Some(Cmdtyp::H07),
            8 => Some(Cmdtyp::H08),
            9 => Some(Cmdtyp::H09),
            _ => None,
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_h00(&self) -> bool {
        *self == Cmdtyp::H00
    }
    #[doc = "Set Endpoint Configuration - -64 or 96-bit Parameter"]
    #[inline(always)]
    pub fn is_h01(&self) -> bool {
        *self == Cmdtyp::H01
    }
    #[doc = "Set Endpoint Transfer Resource Configuration - 32- bitparameter"]
    #[inline(always)]
    pub fn is_h02(&self) -> bool {
        *self == Cmdtyp::H02
    }
    #[doc = "Get Endpoint State - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h03(&self) -> bool {
        *self == Cmdtyp::H03
    }
    #[doc = "Set Stall - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h04(&self) -> bool {
        *self == Cmdtyp::H04
    }
    #[doc = "Clear Stall (see Set Stall) - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h05(&self) -> bool {
        *self == Cmdtyp::H05
    }
    #[doc = "Start Transfer - 64-bit Parameter"]
    #[inline(always)]
    pub fn is_h06(&self) -> bool {
        *self == Cmdtyp::H06
    }
    #[doc = "Update Transfer - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h07(&self) -> bool {
        *self == Cmdtyp::H07
    }
    #[doc = "End Transfer - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h08(&self) -> bool {
        *self == Cmdtyp::H08
    }
    #[doc = "Start New Configuration - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h09(&self) -> bool {
        *self == Cmdtyp::H09
    }
}
#[doc = "Field `CMDTYP` writer - Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform."]
pub type CmdtypW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cmdtyp>;
impl<'a, REG> CmdtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn h00(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H00)
    }
    #[doc = "Set Endpoint Configuration - -64 or 96-bit Parameter"]
    #[inline(always)]
    pub fn h01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H01)
    }
    #[doc = "Set Endpoint Transfer Resource Configuration - 32- bitparameter"]
    #[inline(always)]
    pub fn h02(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H02)
    }
    #[doc = "Get Endpoint State - No Parameter Needed"]
    #[inline(always)]
    pub fn h03(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H03)
    }
    #[doc = "Set Stall - No Parameter Needed"]
    #[inline(always)]
    pub fn h04(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H04)
    }
    #[doc = "Clear Stall (see Set Stall) - No Parameter Needed"]
    #[inline(always)]
    pub fn h05(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H05)
    }
    #[doc = "Start Transfer - 64-bit Parameter"]
    #[inline(always)]
    pub fn h06(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H06)
    }
    #[doc = "Update Transfer - No Parameter Needed"]
    #[inline(always)]
    pub fn h07(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H07)
    }
    #[doc = "End Transfer - No Parameter Needed"]
    #[inline(always)]
    pub fn h08(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H08)
    }
    #[doc = "Start New Configuration - No Parameter Needed"]
    #[inline(always)]
    pub fn h09(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H09)
    }
}
#[doc = "Field `CMDIOC` reader - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a generic\n\nEndpoint Command Complete event after executing the\n\ncommand.\n\nNote that this interrupt is mapped to DEPCFG.IntrNum.\n\nWhen the DEPCFG command is executed, the command interrupt\n\non completion goes to the interrupt pointed by the\n\nDEPCFG.IntrNum in the current command.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
pub type CmdiocR = crate::BitReader;
#[doc = "Field `CMDIOC` writer - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a generic\n\nEndpoint Command Complete event after executing the\n\ncommand.\n\nNote that this interrupt is mapped to DEPCFG.IntrNum.\n\nWhen the DEPCFG command is executed, the command interrupt\n\non completion goes to the interrupt pointed by the\n\nDEPCFG.IntrNum in the current command.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
pub type CmdiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDACT` reader - Command Active\n\nSoftware sets this bit to 1 to enable the device endpoint\n\ncontroller to execute the generic command.\n\nThe device controller sets this bit to 0 when the CmdStatus field\n\nis valid and the endpoint is ready to accept another command.\n\nThis does not imply that all the effects of the previously-issued\n\ncommand have taken place."]
pub type CmdactR = crate::BitReader;
#[doc = "Field `CMDACT` writer - Command Active\n\nSoftware sets this bit to 1 to enable the device endpoint\n\ncontroller to execute the generic command.\n\nThe device controller sets this bit to 0 when the CmdStatus field\n\nis valid and the endpoint is ready to accept another command.\n\nThis does not imply that all the effects of the previously-issued\n\ncommand have taken place."]
pub type CmdactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIPRI_FORCERM` reader - HighPriority/ForceRM\n\nHighPriority: Only valid for Start Transfer command\n\nForceRM: Only valid for End Transfer command\n\nClearPendIN: Only valid for Clear Stall command . Software sets\n\nthis bit to clear any pending IN transaction (on that endpoint)\n\nstuck at the lower layers when a Clear Stall command is issued."]
pub type HipriForcermR = crate::BitReader;
#[doc = "Field `HIPRI_FORCERM` writer - HighPriority/ForceRM\n\nHighPriority: Only valid for Start Transfer command\n\nForceRM: Only valid for End Transfer command\n\nClearPendIN: Only valid for Clear Stall command . Software sets\n\nthis bit to clear any pending IN transaction (on that endpoint)\n\nstuck at the lower layers when a Clear Stall command is issued."]
pub type HipriForcermW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTATUS` reader - Command Completion Status\n\nThe information is in the same format as bits 15:12 of the\n\nEndpoint Command Complete event, Please see \"Device\n\nEndpoint-n Events: DEPEVT\" in the Databook."]
pub type CmdstatusR = crate::FieldReader;
#[doc = "Field `CMDSTATUS` writer - Command Completion Status\n\nThe information is in the same format as bits 15:12 of the\n\nEndpoint Command Complete event, Please see \"Device\n\nEndpoint-n Events: DEPEVT\" in the Databook."]
pub type CmdstatusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMMANDPARAM` reader - Command Parameters or Event Parameters\n\nwhen this register is written:\n\nFor Start Transfer command: The 16-bit StreamID assigned to\n\nthis transfer\n\nFor Start Transfer command applied to an isochronous endpoint:\n\nStartMicroFramNum, Indicates the (micro)frame number to which\n\nthe first TRB applies.\n\nFor Update Transfer, End Transfer, and Start New Configuration\n\ncommands: \\[22:16\\]: Transfer Resource Index (XferRscIdx). The\n\nhardware-assigned transfer resource index for the transfer, which\n\nwas returned in response to the Start Transfer command. The\n\napplication software-assigned transfer resource index for a Start\n\nNew Configuration command.\n\nEvent Parameters (EventParam), when this register is read.\n\nPlease see bits \\[31:16\\]
in the \"Device Endpoint-n Events:\n\nDEPEVT\" of the Databook."]
pub type CommandparamR = crate::FieldReader<u16>;
#[doc = "Field `COMMANDPARAM` writer - Command Parameters or Event Parameters\n\nwhen this register is written:\n\nFor Start Transfer command: The 16-bit StreamID assigned to\n\nthis transfer\n\nFor Start Transfer command applied to an isochronous endpoint:\n\nStartMicroFramNum, Indicates the (micro)frame number to which\n\nthe first TRB applies.\n\nFor Update Transfer, End Transfer, and Start New Configuration\n\ncommands: \\[22:16\\]: Transfer Resource Index (XferRscIdx). The\n\nhardware-assigned transfer resource index for the transfer, which\n\nwas returned in response to the Start Transfer command. The\n\napplication software-assigned transfer resource index for a Start\n\nNew Configuration command.\n\nEvent Parameters (EventParam), when this register is read.\n\nPlease see bits \\[31:16\\]
in the \"Device Endpoint-n Events:\n\nDEPEVT\" of the Databook."]
pub type CommandparamW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform."]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CmdtypR {
        CmdtypR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a generic\n\nEndpoint Command Complete event after executing the\n\ncommand.\n\nNote that this interrupt is mapped to DEPCFG.IntrNum.\n\nWhen the DEPCFG command is executed, the command interrupt\n\non completion goes to the interrupt pointed by the\n\nDEPCFG.IntrNum in the current command.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
    #[inline(always)]
    pub fn cmdioc(&self) -> CmdiocR {
        CmdiocR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Command Active\n\nSoftware sets this bit to 1 to enable the device endpoint\n\ncontroller to execute the generic command.\n\nThe device controller sets this bit to 0 when the CmdStatus field\n\nis valid and the endpoint is ready to accept another command.\n\nThis does not imply that all the effects of the previously-issued\n\ncommand have taken place."]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HighPriority/ForceRM\n\nHighPriority: Only valid for Start Transfer command\n\nForceRM: Only valid for End Transfer command\n\nClearPendIN: Only valid for Clear Stall command . Software sets\n\nthis bit to clear any pending IN transaction (on that endpoint)\n\nstuck at the lower layers when a Clear Stall command is issued."]
    #[inline(always)]
    pub fn hipri_forcerm(&self) -> HipriForcermR {
        HipriForcermR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Command Completion Status\n\nThe information is in the same format as bits 15:12 of the\n\nEndpoint Command Complete event, Please see \"Device\n\nEndpoint-n Events: DEPEVT\" in the Databook."]
    #[inline(always)]
    pub fn cmdstatus(&self) -> CmdstatusR {
        CmdstatusR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Command Parameters or Event Parameters\n\nwhen this register is written:\n\nFor Start Transfer command: The 16-bit StreamID assigned to\n\nthis transfer\n\nFor Start Transfer command applied to an isochronous endpoint:\n\nStartMicroFramNum, Indicates the (micro)frame number to which\n\nthe first TRB applies.\n\nFor Update Transfer, End Transfer, and Start New Configuration\n\ncommands: \\[22:16\\]: Transfer Resource Index (XferRscIdx). The\n\nhardware-assigned transfer resource index for the transfer, which\n\nwas returned in response to the Start Transfer command. The\n\napplication software-assigned transfer resource index for a Start\n\nNew Configuration command.\n\nEvent Parameters (EventParam), when this register is read.\n\nPlease see bits \\[31:16\\]
in the \"Device Endpoint-n Events:\n\nDEPEVT\" of the Databook."]
    #[inline(always)]
    pub fn commandparam(&self) -> CommandparamR {
        CommandparamR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform."]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CmdtypW<Usb3DepcmdSpec> {
        CmdtypW::new(self, 0)
    }
    #[doc = "Bit 8 - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a generic\n\nEndpoint Command Complete event after executing the\n\ncommand.\n\nNote that this interrupt is mapped to DEPCFG.IntrNum.\n\nWhen the DEPCFG command is executed, the command interrupt\n\non completion goes to the interrupt pointed by the\n\nDEPCFG.IntrNum in the current command.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmdioc(&mut self) -> CmdiocW<Usb3DepcmdSpec> {
        CmdiocW::new(self, 8)
    }
    #[doc = "Bit 10 - Command Active\n\nSoftware sets this bit to 1 to enable the device endpoint\n\ncontroller to execute the generic command.\n\nThe device controller sets this bit to 0 when the CmdStatus field\n\nis valid and the endpoint is ready to accept another command.\n\nThis does not imply that all the effects of the previously-issued\n\ncommand have taken place."]
    #[inline(always)]
    #[must_use]
    pub fn cmdact(&mut self) -> CmdactW<Usb3DepcmdSpec> {
        CmdactW::new(self, 10)
    }
    #[doc = "Bit 11 - HighPriority/ForceRM\n\nHighPriority: Only valid for Start Transfer command\n\nForceRM: Only valid for End Transfer command\n\nClearPendIN: Only valid for Clear Stall command . Software sets\n\nthis bit to clear any pending IN transaction (on that endpoint)\n\nstuck at the lower layers when a Clear Stall command is issued."]
    #[inline(always)]
    #[must_use]
    pub fn hipri_forcerm(&mut self) -> HipriForcermW<Usb3DepcmdSpec> {
        HipriForcermW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Command Completion Status\n\nThe information is in the same format as bits 15:12 of the\n\nEndpoint Command Complete event, Please see \"Device\n\nEndpoint-n Events: DEPEVT\" in the Databook."]
    #[inline(always)]
    #[must_use]
    pub fn cmdstatus(&mut self) -> CmdstatusW<Usb3DepcmdSpec> {
        CmdstatusW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Command Parameters or Event Parameters\n\nwhen this register is written:\n\nFor Start Transfer command: The 16-bit StreamID assigned to\n\nthis transfer\n\nFor Start Transfer command applied to an isochronous endpoint:\n\nStartMicroFramNum, Indicates the (micro)frame number to which\n\nthe first TRB applies.\n\nFor Update Transfer, End Transfer, and Start New Configuration\n\ncommands: \\[22:16\\]: Transfer Resource Index (XferRscIdx). The\n\nhardware-assigned transfer resource index for the transfer, which\n\nwas returned in response to the Start Transfer command. The\n\napplication software-assigned transfer resource index for a Start\n\nNew Configuration command.\n\nEvent Parameters (EventParam), when this register is read.\n\nPlease see bits \\[31:16\\]
in the \"Device Endpoint-n Events:\n\nDEPEVT\" of the Databook."]
    #[inline(always)]
    #[must_use]
    pub fn commandparam(&mut self) -> CommandparamW<Usb3DepcmdSpec> {
        CommandparamW::new(self, 16)
    }
}
#[doc = "Device Physical Endpoint-n Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_depcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_depcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3DepcmdSpec;
impl crate::RegisterSpec for Usb3DepcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_depcmd::R`](R) reader structure"]
impl crate::Readable for Usb3DepcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_depcmd::W`](W) writer structure"]
impl crate::Writable for Usb3DepcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_DEP%sCMD to value 0"]
impl crate::Resettable for Usb3DepcmdSpec {
    const RESET_VALUE: u32 = 0;
}
