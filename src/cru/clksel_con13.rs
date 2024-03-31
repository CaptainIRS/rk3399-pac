#[doc = "Register `CLKSEL_CON13` reader"]
pub type R = crate::R<ClkselCon13Spec>;
#[doc = "Register `CLKSEL_CON13` writer"]
pub type W = crate::W<ClkselCon13Spec>;
#[doc = "Field `ACLK_GPU_DIV_CON` reader - aclk_gpu divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkGpuDivConR = crate::FieldReader;
#[doc = "Field `ACLK_GPU_DIV_CON` writer - aclk_gpu divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkGpuDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_gpu clock source select control register\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkGpuPllSel {
    #[doc = "0: PPLL"]
    B000 = 0,
    #[doc = "1: CPLL"]
    B001 = 1,
    #[doc = "2: GPLL"]
    B010 = 2,
    #[doc = "3: NPLL"]
    B011 = 3,
    #[doc = "4: USB_480M"]
    B100 = 4,
}
impl From<AclkGpuPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkGpuPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkGpuPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_GPU_PLL_SEL` reader - aclk_gpu clock source select control register"]
pub type AclkGpuPllSelR = crate::FieldReader<AclkGpuPllSel>;
impl AclkGpuPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkGpuPllSel> {
        match self.bits {
            0 => Some(AclkGpuPllSel::B000),
            1 => Some(AclkGpuPllSel::B001),
            2 => Some(AclkGpuPllSel::B010),
            3 => Some(AclkGpuPllSel::B011),
            4 => Some(AclkGpuPllSel::B100),
            _ => None,
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == AclkGpuPllSel::B000
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == AclkGpuPllSel::B001
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == AclkGpuPllSel::B010
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == AclkGpuPllSel::B011
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == AclkGpuPllSel::B100
    }
}
#[doc = "Field `ACLK_GPU_PLL_SEL` writer - aclk_gpu clock source select control register"]
pub type AclkGpuPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, AclkGpuPllSel>;
impl<'a, REG> AclkGpuPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGpuPllSel::B000)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGpuPllSel::B001)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGpuPllSel::B010)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGpuPllSel::B011)
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGpuPllSel::B100)
    }
}
#[doc = "Field `HCLK_SD_DIV_CON` reader - hclk_sd divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkSdDivConR = crate::FieldReader;
#[doc = "Field `HCLK_SD_DIV_CON` writer - hclk_sd divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkSdDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "hclk_sd clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HclkSdSrcSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<HclkSdSrcSel> for bool {
    #[inline(always)]
    fn from(variant: HclkSdSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCLK_SD_SRC_SEL` reader - hclk_sd clock source select control register"]
pub type HclkSdSrcSelR = crate::BitReader<HclkSdSrcSel>;
impl HclkSdSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HclkSdSrcSel {
        match self.bits {
            false => HclkSdSrcSel::B0,
            true => HclkSdSrcSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HclkSdSrcSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HclkSdSrcSel::B1
    }
}
#[doc = "Field `HCLK_SD_SRC_SEL` writer - hclk_sd clock source select control register"]
pub type HclkSdSrcSelW<'a, REG> = crate::BitWriter<'a, REG, HclkSdSrcSel>;
impl<'a, REG> HclkSdSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HclkSdSrcSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HclkSdSrcSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_gpu divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_gpu_div_con(&self) -> AclkGpuDivConR {
        AclkGpuDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - aclk_gpu clock source select control register"]
    #[inline(always)]
    pub fn aclk_gpu_pll_sel(&self) -> AclkGpuPllSelR {
        AclkGpuPllSelR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - hclk_sd divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_sd_div_con(&self) -> HclkSdDivConR {
        HclkSdDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - hclk_sd clock source select control register"]
    #[inline(always)]
    pub fn hclk_sd_src_sel(&self) -> HclkSdSrcSelR {
        HclkSdSrcSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_gpu divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gpu_div_con(&mut self) -> AclkGpuDivConW<ClkselCon13Spec> {
        AclkGpuDivConW::new(self, 0)
    }
    #[doc = "Bits 5:7 - aclk_gpu clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gpu_pll_sel(&mut self) -> AclkGpuPllSelW<ClkselCon13Spec> {
        AclkGpuPllSelW::new(self, 5)
    }
    #[doc = "Bits 8:12 - hclk_sd divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sd_div_con(&mut self) -> HclkSdDivConW<ClkselCon13Spec> {
        HclkSdDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - hclk_sd clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sd_src_sel(&mut self) -> HclkSdSrcSelW<ClkselCon13Spec> {
        HclkSdSrcSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon13Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon13Spec;
impl crate::RegisterSpec for ClkselCon13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con13::R`](R) reader structure"]
impl crate::Readable for ClkselCon13Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con13::W`](W) writer structure"]
impl crate::Writable for ClkselCon13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON13 to value 0x0361"]
impl crate::Resettable for ClkselCon13Spec {
    const RESET_VALUE: u32 = 0x0361;
}
