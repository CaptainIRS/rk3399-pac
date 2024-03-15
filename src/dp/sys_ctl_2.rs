#[doc = "Register `SYS_CTL_2` reader"]
pub type R = crate::R<SysCtl2Spec>;
#[doc = "Register `SYS_CTL_2` writer"]
pub type W = crate::W<SysCtl2Spec>;
#[doc = "Pixel clock frequency change status control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChaCtrl {
    #[doc = "1: Use auto-detected status This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Use auto-detected status This bit's type is R/W."]
    B0 = 0,
}
impl From<ChaCtrl> for bool {
    #[inline(always)]
    fn from(variant: ChaCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHA_CTRL` reader - Pixel clock frequency change status control"]
pub type ChaCtrlR = crate::BitReader<ChaCtrl>;
impl ChaCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChaCtrl {
        match self.bits {
            true => ChaCtrl::B1,
            false => ChaCtrl::B0,
        }
    }
    #[doc = "Use auto-detected status This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChaCtrl::B1
    }
    #[doc = "Use auto-detected status This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChaCtrl::B0
    }
}
#[doc = "Field `CHA_CTRL` writer - Pixel clock frequency change status control"]
pub type ChaCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, ChaCtrl>;
impl<'a, REG> ChaCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use auto-detected status This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChaCtrl::B1)
    }
    #[doc = "Use auto-detected status This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChaCtrl::B0)
    }
}
#[doc = "Force stream clock change status, this bit only active when CHA_CTRL is 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceCha {
    #[doc = "1: Force clock not change This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Force clock not change This bit's type is R/W."]
    B0 = 0,
}
impl From<ForceCha> for bool {
    #[inline(always)]
    fn from(variant: ForceCha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_CHA` reader - Force stream clock change status, this bit only active when CHA_CTRL is 1"]
pub type ForceChaR = crate::BitReader<ForceCha>;
impl ForceChaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceCha {
        match self.bits {
            true => ForceCha::B1,
            false => ForceCha::B0,
        }
    }
    #[doc = "Force clock not change This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ForceCha::B1
    }
    #[doc = "Force clock not change This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ForceCha::B0
    }
}
#[doc = "Field `FORCE_CHA` writer - Force stream clock change status, this bit only active when CHA_CTRL is 1"]
pub type ForceChaW<'a, REG> = crate::BitWriter1C<'a, REG, ForceCha>;
impl<'a, REG> ForceChaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force clock not change This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceCha::B1)
    }
    #[doc = "Force clock not change This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ForceCha::B0)
    }
}
#[doc = "Video stream clock change status, It will not affect video output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChaSta {
    #[doc = "1: Clock frequency not changed Write any value to update the current status."]
    B1 = 1,
    #[doc = "0: Clock frequency not changed Write any value to update the current status."]
    B0 = 0,
}
impl From<ChaSta> for bool {
    #[inline(always)]
    fn from(variant: ChaSta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHA_STA` reader - Video stream clock change status, It will not affect video output"]
pub type ChaStaR = crate::BitReader<ChaSta>;
impl ChaStaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChaSta {
        match self.bits {
            true => ChaSta::B1,
            false => ChaSta::B0,
        }
    }
    #[doc = "Clock frequency not changed Write any value to update the current status."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChaSta::B1
    }
    #[doc = "Clock frequency not changed Write any value to update the current status."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChaSta::B0
    }
}
#[doc = "Field `CHA_STA` writer - Video stream clock change status, It will not affect video output"]
pub type ChaStaW<'a, REG> = crate::BitWriter1C<'a, REG, ChaSta>;
impl<'a, REG> ChaStaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock frequency not changed Write any value to update the current status."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChaSta::B1)
    }
    #[doc = "Clock frequency not changed Write any value to update the current status."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChaSta::B0)
    }
}
#[doc = "Field `CHA_CRI` reader - Pixel clock change detection threshold. The incoming pixel clock input is counted continuously by the 24Mhz reference clock. This register defines a number, if the counter number change is more than this value for 2 pixel clock edges, the CHA_STA bit is asserted. This bit's type is R/W."]
pub type ChaCriR = crate::FieldReader;
#[doc = "Field `CHA_CRI` writer - Pixel clock change detection threshold. The incoming pixel clock input is counted continuously by the 24Mhz reference clock. This register defines a number, if the counter number change is more than this value for 2 pixel clock edges, the CHA_STA bit is asserted. This bit's type is R/W."]
pub type ChaCriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Pixel clock frequency change status control"]
    #[inline(always)]
    pub fn cha_ctrl(&self) -> ChaCtrlR {
        ChaCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force stream clock change status, this bit only active when CHA_CTRL is 1"]
    #[inline(always)]
    pub fn force_cha(&self) -> ForceChaR {
        ForceChaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Video stream clock change status, It will not affect video output"]
    #[inline(always)]
    pub fn cha_sta(&self) -> ChaStaR {
        ChaStaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Pixel clock change detection threshold. The incoming pixel clock input is counted continuously by the 24Mhz reference clock. This register defines a number, if the counter number change is more than this value for 2 pixel clock edges, the CHA_STA bit is asserted. This bit's type is R/W."]
    #[inline(always)]
    pub fn cha_cri(&self) -> ChaCriR {
        ChaCriR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pixel clock frequency change status control"]
    #[inline(always)]
    #[must_use]
    pub fn cha_ctrl(&mut self) -> ChaCtrlW<SysCtl2Spec> {
        ChaCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Force stream clock change status, this bit only active when CHA_CTRL is 1"]
    #[inline(always)]
    #[must_use]
    pub fn force_cha(&mut self) -> ForceChaW<SysCtl2Spec> {
        ForceChaW::new(self, 1)
    }
    #[doc = "Bit 2 - Video stream clock change status, It will not affect video output"]
    #[inline(always)]
    #[must_use]
    pub fn cha_sta(&mut self) -> ChaStaW<SysCtl2Spec> {
        ChaStaW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Pixel clock change detection threshold. The incoming pixel clock input is counted continuously by the 24Mhz reference clock. This register defines a number, if the counter number change is more than this value for 2 pixel clock edges, the CHA_STA bit is asserted. This bit's type is R/W."]
    #[inline(always)]
    #[must_use]
    pub fn cha_cri(&mut self) -> ChaCriW<SysCtl2Spec> {
        ChaCriW::new(self, 4)
    }
}
#[doc = "System Control Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtl2Spec;
impl crate::RegisterSpec for SysCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctl_2::R`](R) reader structure"]
impl crate::Readable for SysCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctl_2::W`](W) writer structure"]
impl crate::Writable for SysCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf7;
}
#[doc = "`reset()` method sets SYS_CTL_2 to value 0x40"]
impl crate::Resettable for SysCtl2Spec {
    const RESET_VALUE: u32 = 0x40;
}
