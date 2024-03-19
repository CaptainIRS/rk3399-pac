#[doc = "Register `DP_M_CAL_CTL` reader"]
pub type R = crate::R<DpMCalCtlSpec>;
#[doc = "Register `DP_M_CAL_CTL` writer"]
pub type W = crate::W<DpMCalCtlSpec>;
#[doc = "Select which link clock is used to generate \n\nthe M value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MGenClkSel {
    #[doc = "1: Clock with down spreading is used"]
    B1 = 1,
    #[doc = "0: Clock without down spreading is used"]
    B0 = 0,
}
impl From<MGenClkSel> for bool {
    #[inline(always)]
    fn from(variant: MGenClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M_GEN_CLK_SEL` reader - Select which link clock is used to generate \n\nthe M value"]
pub type MGenClkSelR = crate::BitReader<MGenClkSel>;
impl MGenClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MGenClkSel {
        match self.bits {
            true => MGenClkSel::B1,
            false => MGenClkSel::B0,
        }
    }
    #[doc = "Clock with down spreading is used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MGenClkSel::B1
    }
    #[doc = "Clock without down spreading is used"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MGenClkSel::B0
    }
}
#[doc = "Field `M_GEN_CLK_SEL` writer - Select which link clock is used to generate \n\nthe M value"]
pub type MGenClkSelW<'a, REG> = crate::BitWriter1C<'a, REG, MGenClkSel>;
impl<'a, REG> MGenClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock with down spreading is used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MGenClkSel::B1)
    }
    #[doc = "Clock without down spreading is used"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MGenClkSel::B0)
    }
}
#[doc = "Enable M_VID value generation filter to \n\nreduce the variation of M_VID value. This \n\nfilter is a low-pass filter to smooth out the \n\nM_VID variation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MVidGenFilterEn {
    #[doc = "1: Enable the filter"]
    B1 = 1,
    #[doc = "0: Disable the filter Note: Refer to page 22 for details."]
    B0 = 0,
}
impl From<MVidGenFilterEn> for bool {
    #[inline(always)]
    fn from(variant: MVidGenFilterEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M_VID_GEN_FILTER_EN` reader - Enable M_VID value generation filter to \n\nreduce the variation of M_VID value. This \n\nfilter is a low-pass filter to smooth out the \n\nM_VID variation"]
pub type MVidGenFilterEnR = crate::BitReader<MVidGenFilterEn>;
impl MVidGenFilterEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MVidGenFilterEn {
        match self.bits {
            true => MVidGenFilterEn::B1,
            false => MVidGenFilterEn::B0,
        }
    }
    #[doc = "Enable the filter"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MVidGenFilterEn::B1
    }
    #[doc = "Disable the filter Note: Refer to page 22 for details."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MVidGenFilterEn::B0
    }
}
#[doc = "Field `M_VID_GEN_FILTER_EN` writer - Enable M_VID value generation filter to \n\nreduce the variation of M_VID value. This \n\nfilter is a low-pass filter to smooth out the \n\nM_VID variation"]
pub type MVidGenFilterEnW<'a, REG> = crate::BitWriter<'a, REG, MVidGenFilterEn>;
impl<'a, REG> MVidGenFilterEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the filter"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MVidGenFilterEn::B1)
    }
    #[doc = "Disable the filter Note: Refer to page 22 for details."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MVidGenFilterEn::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Select which link clock is used to generate \n\nthe M value"]
    #[inline(always)]
    pub fn m_gen_clk_sel(&self) -> MGenClkSelR {
        MGenClkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable M_VID value generation filter to \n\nreduce the variation of M_VID value. This \n\nfilter is a low-pass filter to smooth out the \n\nM_VID variation"]
    #[inline(always)]
    pub fn m_vid_gen_filter_en(&self) -> MVidGenFilterEnR {
        MVidGenFilterEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select which link clock is used to generate \n\nthe M value"]
    #[inline(always)]
    #[must_use]
    pub fn m_gen_clk_sel(&mut self) -> MGenClkSelW<DpMCalCtlSpec> {
        MGenClkSelW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable M_VID value generation filter to \n\nreduce the variation of M_VID value. This \n\nfilter is a low-pass filter to smooth out the \n\nM_VID variation"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_gen_filter_en(&mut self) -> MVidGenFilterEnW<DpMCalCtlSpec> {
        MVidGenFilterEnW::new(self, 2)
    }
}
#[doc = "DP M Value Calculation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_m_cal_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_m_cal_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpMCalCtlSpec;
impl crate::RegisterSpec for DpMCalCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_m_cal_ctl::R`](R) reader structure"]
impl crate::Readable for DpMCalCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_m_cal_ctl::W`](W) writer structure"]
impl crate::Writable for DpMCalCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets DP_M_CAL_CTL to value 0"]
impl crate::Resettable for DpMCalCtlSpec {
    const RESET_VALUE: u32 = 0;
}
