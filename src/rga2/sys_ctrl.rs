#[doc = "Register `SYS_CTRL` reader"]
pub type R = crate::R<SysCtrlSpec>;
#[doc = "Register `SYS_CTRL` writer"]
pub type W = crate::W<SysCtrlSpec>;
#[doc = "Field `SW_CMD_OP_ST_P` reader - RGA operation start bit\n\nOnly used in passive (slave) control mode"]
pub type SwCmdOpStPR = crate::BitReader;
#[doc = "Field `SW_CMD_OP_ST_P` writer - RGA operation start bit\n\nOnly used in passive (slave) control mode"]
pub type SwCmdOpStPW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "RGA command mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCmdMode {
    #[doc = "0: slave mode"]
    B0 = 0,
    #[doc = "1: master mode"]
    B1 = 1,
}
impl From<SwCmdMode> for bool {
    #[inline(always)]
    fn from(variant: SwCmdMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CMD_MODE` writer - RGA command mode"]
pub type SwCmdModeW<'a, REG> = crate::BitWriter<'a, REG, SwCmdMode>;
impl<'a, REG> SwCmdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCmdMode::B0)
    }
    #[doc = "master mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCmdMode::B1)
    }
}
#[doc = "RGA auto clock gating enable bit\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAutoCkg {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwAutoCkg> for bool {
    #[inline(always)]
    fn from(variant: SwAutoCkg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_AUTO_CKG` writer - RGA auto clock gating enable bit"]
pub type SwAutoCkgW<'a, REG> = crate::BitWriter<'a, REG, SwAutoCkg>;
impl<'a, REG> SwAutoCkgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAutoCkg::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAutoCkg::B1)
    }
}
#[doc = "Field `SW_ACLK_SRESET_P` writer - RGA aclk domain Soft reset, write '1' to this would reset the RGA\n\nengine except config registers."]
pub type SwAclkSresetPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CCLK_SRESET_P` reader - RGA core clk domain Soft reset, write '1' to this would reset the RGA\n\nengine except config registers."]
pub type SwCclkSresetPR = crate::BitReader;
#[doc = "Field `SW_CCLK_SRESET_P` writer - RGA core clk domain Soft reset, write '1' to this would reset the RGA\n\nengine except config registers."]
pub type SwCclkSresetPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "it would auto-resetn after one frame finish.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAutoRst {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwAutoRst> for bool {
    #[inline(always)]
    fn from(variant: SwAutoRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_AUTO_RST` reader - it would auto-resetn after one frame finish."]
pub type SwAutoRstR = crate::BitReader<SwAutoRst>;
impl SwAutoRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAutoRst {
        match self.bits {
            false => SwAutoRst::B0,
            true => SwAutoRst::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAutoRst::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAutoRst::B1
    }
}
#[doc = "Field `SW_AUTO_RST` writer - it would auto-resetn after one frame finish."]
pub type SwAutoRstW<'a, REG> = crate::BitWriter<'a, REG, SwAutoRst>;
impl<'a, REG> SwAutoRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAutoRst::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAutoRst::B1)
    }
}
#[doc = "Field `SW_RST_PROTECT_E` reader - protect-rstn mode enable.\n\nit would be ensure all axi write/read operation into completion\n\nstatus when sw_cclk_sreset_p or sw_aclk_sreset_p valid."]
pub type SwRstProtectER = crate::BitReader;
#[doc = "Field `SW_RST_PROTECT_E` writer - protect-rstn mode enable.\n\nit would be ensure all axi write/read operation into completion\n\nstatus when sw_cclk_sreset_p or sw_aclk_sreset_p valid."]
pub type SwRstProtectEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_HANDSAVE_P` reader - it would save protect-rstn into initial status if long time dead in\n\nprotect-rstn status. (auto clear into '0')"]
pub type SwRstHandsavePR = crate::BitReader;
#[doc = "Field `SW_RST_HANDSAVE_P` writer - it would save protect-rstn into initial status if long time dead in\n\nprotect-rstn status. (auto clear into '0')"]
pub type SwRstHandsavePW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RGA operation start bit\n\nOnly used in passive (slave) control mode"]
    #[inline(always)]
    pub fn sw_cmd_op_st_p(&self) -> SwCmdOpStPR {
        SwCmdOpStPR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - RGA core clk domain Soft reset, write '1' to this would reset the RGA\n\nengine except config registers."]
    #[inline(always)]
    pub fn sw_cclk_sreset_p(&self) -> SwCclkSresetPR {
        SwCclkSresetPR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - it would auto-resetn after one frame finish."]
    #[inline(always)]
    pub fn sw_auto_rst(&self) -> SwAutoRstR {
        SwAutoRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - protect-rstn mode enable.\n\nit would be ensure all axi write/read operation into completion\n\nstatus when sw_cclk_sreset_p or sw_aclk_sreset_p valid."]
    #[inline(always)]
    pub fn sw_rst_protect_e(&self) -> SwRstProtectER {
        SwRstProtectER::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - it would save protect-rstn into initial status if long time dead in\n\nprotect-rstn status. (auto clear into '0')"]
    #[inline(always)]
    pub fn sw_rst_handsave_p(&self) -> SwRstHandsavePR {
        SwRstHandsavePR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGA operation start bit\n\nOnly used in passive (slave) control mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_op_st_p(&mut self) -> SwCmdOpStPW<SysCtrlSpec> {
        SwCmdOpStPW::new(self, 0)
    }
    #[doc = "Bit 1 - RGA command mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_mode(&mut self) -> SwCmdModeW<SysCtrlSpec> {
        SwCmdModeW::new(self, 1)
    }
    #[doc = "Bit 2 - RGA auto clock gating enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sw_auto_ckg(&mut self) -> SwAutoCkgW<SysCtrlSpec> {
        SwAutoCkgW::new(self, 2)
    }
    #[doc = "Bit 3 - RGA aclk domain Soft reset, write '1' to this would reset the RGA\n\nengine except config registers."]
    #[inline(always)]
    #[must_use]
    pub fn sw_aclk_sreset_p(&mut self) -> SwAclkSresetPW<SysCtrlSpec> {
        SwAclkSresetPW::new(self, 3)
    }
    #[doc = "Bit 4 - RGA core clk domain Soft reset, write '1' to this would reset the RGA\n\nengine except config registers."]
    #[inline(always)]
    #[must_use]
    pub fn sw_cclk_sreset_p(&mut self) -> SwCclkSresetPW<SysCtrlSpec> {
        SwCclkSresetPW::new(self, 4)
    }
    #[doc = "Bit 5 - it would auto-resetn after one frame finish."]
    #[inline(always)]
    #[must_use]
    pub fn sw_auto_rst(&mut self) -> SwAutoRstW<SysCtrlSpec> {
        SwAutoRstW::new(self, 5)
    }
    #[doc = "Bit 6 - protect-rstn mode enable.\n\nit would be ensure all axi write/read operation into completion\n\nstatus when sw_cclk_sreset_p or sw_aclk_sreset_p valid."]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_protect_e(&mut self) -> SwRstProtectEW<SysCtrlSpec> {
        SwRstProtectEW::new(self, 6)
    }
    #[doc = "Bit 7 - it would save protect-rstn into initial status if long time dead in\n\nprotect-rstn status. (auto clear into '0')"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_handsave_p(&mut self) -> SwRstHandsavePW<SysCtrlSpec> {
        SwRstHandsavePW::new(self, 7)
    }
}
#[doc = "RGA system control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtrlSpec;
impl crate::RegisterSpec for SysCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl::R`](R) reader structure"]
impl crate::Readable for SysCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl::W`](W) writer structure"]
impl crate::Writable for SysCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0x44"]
impl crate::Resettable for SysCtrlSpec {
    const RESET_VALUE: u32 = 0x44;
}
