#[doc = "Register `CLKSEL_CON45` reader"]
pub type R = crate::R<ClkselCon45Spec>;
#[doc = "Register `CLKSEL_CON45` writer"]
pub type W = crate::W<ClkselCon45Spec>;
#[doc = "Field `CLK_HDMI_CEC_DIV_CON` reader - clk_hdmi_cec divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkHdmiCecDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLK_HDMI_CEC_DIV_CON` writer - clk_hdmi_cec divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkHdmiCecDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "clk_hdmi_cec_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkHdmiCecSrcSel {
    #[doc = "0: clk_32k"]
    B0 = 0,
    #[doc = "1: xin_24m"]
    B1 = 1,
}
impl From<ClkHdmiCecSrcSel> for bool {
    #[inline(always)]
    fn from(variant: ClkHdmiCecSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_HDMI_CEC_SRC_SEL` reader - clk_hdmi_cec_src clock select control register"]
pub type ClkHdmiCecSrcSelR = crate::BitReader<ClkHdmiCecSrcSel>;
impl ClkHdmiCecSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkHdmiCecSrcSel {
        match self.bits {
            false => ClkHdmiCecSrcSel::B0,
            true => ClkHdmiCecSrcSel::B1,
        }
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkHdmiCecSrcSel::B0
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkHdmiCecSrcSel::B1
    }
}
#[doc = "Field `CLK_HDMI_CEC_SRC_SEL` writer - clk_hdmi_cec_src clock select control register"]
pub type ClkHdmiCecSrcSelW<'a, REG> = crate::BitWriter<'a, REG, ClkHdmiCecSrcSel>;
impl<'a, REG> ClkHdmiCecSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkHdmiCecSrcSel::B0)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkHdmiCecSrcSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - clk_hdmi_cec divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_hdmi_cec_div_con(&self) -> ClkHdmiCecDivConR {
        ClkHdmiCecDivConR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - clk_hdmi_cec_src clock select control register"]
    #[inline(always)]
    pub fn clk_hdmi_cec_src_sel(&self) -> ClkHdmiCecSrcSelR {
        ClkHdmiCecSrcSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - clk_hdmi_cec divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hdmi_cec_div_con(&mut self) -> ClkHdmiCecDivConW<ClkselCon45Spec> {
        ClkHdmiCecDivConW::new(self, 0)
    }
    #[doc = "Bit 15 - clk_hdmi_cec_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hdmi_cec_src_sel(&mut self) -> ClkHdmiCecSrcSelW<ClkselCon45Spec> {
        ClkHdmiCecSrcSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon45Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon45Spec;
impl crate::RegisterSpec for ClkselCon45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con45::R`](R) reader structure"]
impl crate::Readable for ClkselCon45Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con45::W`](W) writer structure"]
impl crate::Writable for ClkselCon45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON45 to value 0x02dc"]
impl crate::Resettable for ClkselCon45Spec {
    const RESET_VALUE: u32 = 0x02dc;
}
