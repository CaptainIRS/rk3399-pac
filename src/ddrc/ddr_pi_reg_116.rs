#[doc = "Register `DDR_PI_REG_116` reader"]
pub type R = crate::R<DdrPiReg116Spec>;
#[doc = "Register `DDR_PI_REG_116` writer"]
pub type W = crate::W<DdrPiReg116Spec>;
#[doc = "Indicates dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PiDramClkDisableDeassertSel {
    #[doc = "0: dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    B0 = 0,
    #[doc = "1: dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    B1 = 1,
}
impl From<PiDramClkDisableDeassertSel> for bool {
    #[inline(always)]
    fn from(variant: PiDramClkDisableDeassertSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI_DRAM_CLK_DISABLE_DEASSERT_SEL` reader - Indicates dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert."]
pub type PiDramClkDisableDeassertSelR = crate::BitReader<PiDramClkDisableDeassertSel>;
impl PiDramClkDisableDeassertSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PiDramClkDisableDeassertSel {
        match self.bits {
            false => PiDramClkDisableDeassertSel::B0,
            true => PiDramClkDisableDeassertSel::B1,
        }
    }
    #[doc = "dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PiDramClkDisableDeassertSel::B0
    }
    #[doc = "dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PiDramClkDisableDeassertSel::B1
    }
}
#[doc = "Field `PI_DRAM_CLK_DISABLE_DEASSERT_SEL` writer - Indicates dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert."]
pub type PiDramClkDisableDeassertSelW<'a, REG> =
    crate::BitWriter<'a, REG, PiDramClkDisableDeassertSel>;
impl<'a, REG> PiDramClkDisableDeassertSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PiDramClkDisableDeassertSel::B0)
    }
    #[doc = "dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PiDramClkDisableDeassertSel::B1)
    }
}
#[doc = "Field `PI_REFRESH_BETWEEN_SEGMENT_DISABLE` reader - Disables the refresh between CA first and second segment training. Defaut is set to 1."]
pub type PiRefreshBetweenSegmentDisableR = crate::BitReader;
#[doc = "Field `PI_REFRESH_BETWEEN_SEGMENT_DISABLE` writer - Disables the refresh between CA first and second segment training. Defaut is set to 1."]
pub type PiRefreshBetweenSegmentDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TCKEHDQS_F0` reader - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTckehdqsF0R = crate::FieldReader;
#[doc = "Field `PI_TCKEHDQS_F0` writer - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTckehdqsF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TCKEHDQS_F1` reader - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTckehdqsF1R = crate::FieldReader;
#[doc = "Field `PI_TCKEHDQS_F1` writer - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTckehdqsF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Indicates dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert."]
    #[inline(always)]
    pub fn pi_dram_clk_disable_deassert_sel(&self) -> PiDramClkDisableDeassertSelR {
        PiDramClkDisableDeassertSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Disables the refresh between CA first and second segment training. Defaut is set to 1."]
    #[inline(always)]
    pub fn pi_refresh_between_segment_disable(&self) -> PiRefreshBetweenSegmentDisableR {
        PiRefreshBetweenSegmentDisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tckehdqs_f0(&self) -> PiTckehdqsF0R {
        PiTckehdqsF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tckehdqs_f1(&self) -> PiTckehdqsF1R {
        PiTckehdqsF1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dram_clk_disable_deassert_sel(
        &mut self,
    ) -> PiDramClkDisableDeassertSelW<DdrPiReg116Spec> {
        PiDramClkDisableDeassertSelW::new(self, 0)
    }
    #[doc = "Bit 8 - Disables the refresh between CA first and second segment training. Defaut is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_refresh_between_segment_disable(
        &mut self,
    ) -> PiRefreshBetweenSegmentDisableW<DdrPiReg116Spec> {
        PiRefreshBetweenSegmentDisableW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckehdqs_f0(&mut self) -> PiTckehdqsF0W<DdrPiReg116Spec> {
        PiTckehdqsF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckehdqs_f1(&mut self) -> PiTckehdqsF1W<DdrPiReg116Spec> {
        PiTckehdqsF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_116::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_116::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg116Spec;
impl crate::RegisterSpec for DdrPiReg116Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_116::R`](R) reader structure"]
impl crate::Readable for DdrPiReg116Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_116::W`](W) writer structure"]
impl crate::Writable for DdrPiReg116Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_116 to value 0"]
impl crate::Resettable for DdrPiReg116Spec {
    const RESET_VALUE: u32 = 0;
}
