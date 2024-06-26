#[doc = "Register `DGCMD` reader"]
pub type R = crate::R<DgcmdSpec>;
#[doc = "Register `DGCMD` writer"]
pub type W = crate::W<DgcmdSpec>;
#[doc = "Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtyp {
    #[doc = "1: Set Endpoint Configuration - 64 or 96-bit Parameter"]
    H01 = 1,
    #[doc = "2: Set Endpoint Transfer Resource Configuration - 32-bit Parameter"]
    H02 = 2,
    #[doc = "3: Get Endpoint State - No Parameter Needed"]
    H03 = 3,
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
            1 => Some(Cmdtyp::H01),
            2 => Some(Cmdtyp::H02),
            3 => Some(Cmdtyp::H03),
            5 => Some(Cmdtyp::H05),
            6 => Some(Cmdtyp::H06),
            7 => Some(Cmdtyp::H07),
            8 => Some(Cmdtyp::H08),
            9 => Some(Cmdtyp::H09),
            _ => None,
        }
    }
    #[doc = "Set Endpoint Configuration - 64 or 96-bit Parameter"]
    #[inline(always)]
    pub fn is_h01(&self) -> bool {
        *self == Cmdtyp::H01
    }
    #[doc = "Set Endpoint Transfer Resource Configuration - 32-bit Parameter"]
    #[inline(always)]
    pub fn is_h02(&self) -> bool {
        *self == Cmdtyp::H02
    }
    #[doc = "Get Endpoint State - No Parameter Needed"]
    #[inline(always)]
    pub fn is_h03(&self) -> bool {
        *self == Cmdtyp::H03
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
pub type CmdtypW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cmdtyp>;
impl<'a, REG> CmdtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set Endpoint Configuration - 64 or 96-bit Parameter"]
    #[inline(always)]
    pub fn h01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H01)
    }
    #[doc = "Set Endpoint Transfer Resource Configuration - 32-bit Parameter"]
    #[inline(always)]
    pub fn h02(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H02)
    }
    #[doc = "Get Endpoint State - No Parameter Needed"]
    #[inline(always)]
    pub fn h03(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::H03)
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
#[doc = "Field `CMDIOC` reader - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a Generic\n\nCommand Completion event after executing the command.\n\nNote that this interrupt is mapped to DCFG.IntrNum.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
pub type CmdiocR = crate::BitReader;
#[doc = "Field `CMDIOC` writer - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a Generic\n\nCommand Completion event after executing the command.\n\nNote that this interrupt is mapped to DCFG.IntrNum.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
pub type CmdiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDACT` reader - Command Active\n\nThe software sets this bit to 1 to enable the device controller to\n\nexecute the generic command.\n\nThe device controller sets this bit to 0 after executing the\n\ncommand."]
pub type CmdactR = crate::BitReader;
#[doc = "Field `CMDACT` writer - Command Active\n\nThe software sets this bit to 1 to enable the device controller to\n\nexecute the generic command.\n\nThe device controller sets this bit to 0 after executing the\n\ncommand."]
pub type CmdactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Command Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdstatus {
    #[doc = "1: CmdErr: Indicates that the device controller encountered an error while processing the command."]
    B1 = 1,
    #[doc = "0: Indicates command success"]
    B0 = 0,
}
impl From<Cmdstatus> for u8 {
    #[inline(always)]
    fn from(variant: Cmdstatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdstatus {
    type Ux = u8;
}
#[doc = "Field `CMDSTATUS` reader - Command Status"]
pub type CmdstatusR = crate::FieldReader<Cmdstatus>;
impl CmdstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdstatus> {
        match self.bits {
            1 => Some(Cmdstatus::B1),
            0 => Some(Cmdstatus::B0),
            _ => None,
        }
    }
    #[doc = "CmdErr: Indicates that the device controller encountered an error while processing the command."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdstatus::B1
    }
    #[doc = "Indicates command success"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdstatus::B0
    }
}
impl R {
    #[doc = "Bits 0:7 - Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform."]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CmdtypR {
        CmdtypR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a Generic\n\nCommand Completion event after executing the command.\n\nNote that this interrupt is mapped to DCFG.IntrNum.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
    #[inline(always)]
    pub fn cmdioc(&self) -> CmdiocR {
        CmdiocR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Command Active\n\nThe software sets this bit to 1 to enable the device controller to\n\nexecute the generic command.\n\nThe device controller sets this bit to 0 after executing the\n\ncommand."]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Command Status"]
    #[inline(always)]
    pub fn cmdstatus(&self) -> CmdstatusR {
        CmdstatusR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Type\n\nSpecifies the type of command the software driver is requesting\n\nthe core to perform."]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CmdtypW<DgcmdSpec> {
        CmdtypW::new(self, 0)
    }
    #[doc = "Bit 8 - Command Interrupt on Complete\n\nWhen this bit is set, the device controller issues a Generic\n\nCommand Completion event after executing the command.\n\nNote that this interrupt is mapped to DCFG.IntrNum.\n\nNote: This field must not set to 1 if the DCTL.RunStop field is 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmdioc(&mut self) -> CmdiocW<DgcmdSpec> {
        CmdiocW::new(self, 8)
    }
    #[doc = "Bit 10 - Command Active\n\nThe software sets this bit to 1 to enable the device controller to\n\nexecute the generic command.\n\nThe device controller sets this bit to 0 after executing the\n\ncommand."]
    #[inline(always)]
    #[must_use]
    pub fn cmdact(&mut self) -> CmdactW<DgcmdSpec> {
        CmdactW::new(self, 10)
    }
}
#[doc = "Device Generic Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dgcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dgcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DgcmdSpec;
impl crate::RegisterSpec for DgcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dgcmd::R`](R) reader structure"]
impl crate::Readable for DgcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`dgcmd::W`](W) writer structure"]
impl crate::Writable for DgcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DGCMD to value 0"]
impl crate::Resettable for DgcmdSpec {
    const RESET_VALUE: u32 = 0;
}
