#[doc = "Register `MCU_CTRL` reader"]
pub type R = crate::R<McuCtrlSpec>;
#[doc = "Register `MCU_CTRL` writer"]
pub type W = crate::W<McuCtrlSpec>;
#[doc = "Field `MCU_PIX_TOTAL` reader - MCU LCD Interface writing period (1-63)"]
pub type McuPixTotalR = crate::FieldReader;
#[doc = "Field `MCU_PIX_TOTAL` writer - MCU LCD Interface writing period (1-63)"]
pub type McuPixTotalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCU_CS_PST` reader - MCU_CS signal start point (0-15)"]
pub type McuCsPstR = crate::FieldReader;
#[doc = "Field `MCU_CS_PST` writer - MCU_CS signal start point (0-15)"]
pub type McuCsPstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_CS_PEND` reader - MCU_CS signal end point (0-63)"]
pub type McuCsPendR = crate::FieldReader;
#[doc = "Field `MCU_CS_PEND` writer - MCU_CS signal end point (0-63)"]
pub type McuCsPendW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCU_RW_PST` reader - MCU_RW signal start point (0-15)"]
pub type McuRwPstR = crate::FieldReader;
#[doc = "Field `MCU_RW_PST` writer - MCU_RW signal start point (0-15)"]
pub type McuRwPstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_RW_PEND` reader - MCU_RW signal end point (0-63)"]
pub type McuRwPendR = crate::FieldReader;
#[doc = "Field `MCU_RW_PEND` writer - MCU_RW signal end point (0-63)"]
pub type McuRwPendW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "MCU_CLK_SEL for MCU bypass\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McuClkSel {
    #[doc = "1: MCU BYPASS sync with DCLK"]
    B1 = 1,
    #[doc = "0: MCU BYPASS sync with HCLK"]
    B0 = 0,
}
impl From<McuClkSel> for bool {
    #[inline(always)]
    fn from(variant: McuClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCU_CLK_SEL` reader - MCU_CLK_SEL for MCU bypass"]
pub type McuClkSelR = crate::BitReader<McuClkSel>;
impl McuClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McuClkSel {
        match self.bits {
            true => McuClkSel::B1,
            false => McuClkSel::B0,
        }
    }
    #[doc = "MCU BYPASS sync with DCLK"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == McuClkSel::B1
    }
    #[doc = "MCU BYPASS sync with HCLK"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == McuClkSel::B0
    }
}
#[doc = "Field `MCU_CLK_SEL` writer - MCU_CLK_SEL for MCU bypass"]
pub type McuClkSelW<'a, REG> = crate::BitWriter<'a, REG, McuClkSel>;
impl<'a, REG> McuClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCU BYPASS sync with DCLK"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(McuClkSel::B1)
    }
    #[doc = "MCU BYPASS sync with HCLK"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(McuClkSel::B0)
    }
}
#[doc = "Field `MCU_HOLD_MODE` reader - MCU HOLD Mode Select"]
pub type McuHoldModeR = crate::BitReader;
#[doc = "Field `MCU_HOLD_MODE` writer - MCU HOLD Mode Select"]
pub type McuHoldModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_FRAME_ST` reader - Write'1' : MCU HOLD Mode Frame Start\n\nRead : MCU/LCDC standby HOLD status"]
pub type McuFrameStR = crate::BitReader;
#[doc = "Field `MCU_FRAME_ST` writer - Write'1' : MCU HOLD Mode Frame Start\n\nRead : MCU/LCDC standby HOLD status"]
pub type McuFrameStW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MCU_RS` reader - MCU LCD RS Select"]
pub type McuRsR = crate::BitReader;
#[doc = "Field `MCU_RS` writer - MCU LCD RS Select"]
pub type McuRsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_BYPASS` reader - MCU LCD BYPASS MODE Select"]
pub type McuBypassR = crate::BitReader;
#[doc = "Field `MCU_BYPASS` writer - MCU LCD BYPASS MODE Select"]
pub type McuBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_TYPE` reader - MCU LCD output SELECT"]
pub type McuTypeR = crate::BitReader;
#[doc = "Field `MCU_TYPE` writer - MCU LCD output SELECT"]
pub type McuTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - MCU LCD Interface writing period (1-63)"]
    #[inline(always)]
    pub fn mcu_pix_total(&self) -> McuPixTotalR {
        McuPixTotalR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - MCU_CS signal start point (0-15)"]
    #[inline(always)]
    pub fn mcu_cs_pst(&self) -> McuCsPstR {
        McuCsPstR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - MCU_CS signal end point (0-63)"]
    #[inline(always)]
    pub fn mcu_cs_pend(&self) -> McuCsPendR {
        McuCsPendR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - MCU_RW signal start point (0-15)"]
    #[inline(always)]
    pub fn mcu_rw_pst(&self) -> McuRwPstR {
        McuRwPstR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - MCU_RW signal end point (0-63)"]
    #[inline(always)]
    pub fn mcu_rw_pend(&self) -> McuRwPendR {
        McuRwPendR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - MCU_CLK_SEL for MCU bypass"]
    #[inline(always)]
    pub fn mcu_clk_sel(&self) -> McuClkSelR {
        McuClkSelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MCU HOLD Mode Select"]
    #[inline(always)]
    pub fn mcu_hold_mode(&self) -> McuHoldModeR {
        McuHoldModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write'1' : MCU HOLD Mode Frame Start\n\nRead : MCU/LCDC standby HOLD status"]
    #[inline(always)]
    pub fn mcu_frame_st(&self) -> McuFrameStR {
        McuFrameStR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MCU LCD RS Select"]
    #[inline(always)]
    pub fn mcu_rs(&self) -> McuRsR {
        McuRsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MCU LCD BYPASS MODE Select"]
    #[inline(always)]
    pub fn mcu_bypass(&self) -> McuBypassR {
        McuBypassR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MCU LCD output SELECT"]
    #[inline(always)]
    pub fn mcu_type(&self) -> McuTypeR {
        McuTypeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - MCU LCD Interface writing period (1-63)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pix_total(&mut self) -> McuPixTotalW<McuCtrlSpec> {
        McuPixTotalW::new(self, 0)
    }
    #[doc = "Bits 6:9 - MCU_CS signal start point (0-15)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_cs_pst(&mut self) -> McuCsPstW<McuCtrlSpec> {
        McuCsPstW::new(self, 6)
    }
    #[doc = "Bits 10:15 - MCU_CS signal end point (0-63)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_cs_pend(&mut self) -> McuCsPendW<McuCtrlSpec> {
        McuCsPendW::new(self, 10)
    }
    #[doc = "Bits 16:19 - MCU_RW signal start point (0-15)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_rw_pst(&mut self) -> McuRwPstW<McuCtrlSpec> {
        McuRwPstW::new(self, 16)
    }
    #[doc = "Bits 20:25 - MCU_RW signal end point (0-63)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_rw_pend(&mut self) -> McuRwPendW<McuCtrlSpec> {
        McuRwPendW::new(self, 20)
    }
    #[doc = "Bit 26 - MCU_CLK_SEL for MCU bypass"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_clk_sel(&mut self) -> McuClkSelW<McuCtrlSpec> {
        McuClkSelW::new(self, 26)
    }
    #[doc = "Bit 27 - MCU HOLD Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_hold_mode(&mut self) -> McuHoldModeW<McuCtrlSpec> {
        McuHoldModeW::new(self, 27)
    }
    #[doc = "Bit 28 - Write'1' : MCU HOLD Mode Frame Start\n\nRead : MCU/LCDC standby HOLD status"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_frame_st(&mut self) -> McuFrameStW<McuCtrlSpec> {
        McuFrameStW::new(self, 28)
    }
    #[doc = "Bit 29 - MCU LCD RS Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_rs(&mut self) -> McuRsW<McuCtrlSpec> {
        McuRsW::new(self, 29)
    }
    #[doc = "Bit 30 - MCU LCD BYPASS MODE Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_bypass(&mut self) -> McuBypassW<McuCtrlSpec> {
        McuBypassW::new(self, 30)
    }
    #[doc = "Bit 31 - MCU LCD output SELECT"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_type(&mut self) -> McuTypeW<McuCtrlSpec> {
        McuTypeW::new(self, 31)
    }
}
#[doc = "MCU mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuCtrlSpec;
impl crate::RegisterSpec for McuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_ctrl::R`](R) reader structure"]
impl crate::Readable for McuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcu_ctrl::W`](W) writer structure"]
impl crate::Writable for McuCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1000_0000;
}
#[doc = "`reset()` method sets MCU_CTRL to value 0x0071_1c08"]
impl crate::Resettable for McuCtrlSpec {
    const RESET_VALUE: u32 = 0x0071_1c08;
}
