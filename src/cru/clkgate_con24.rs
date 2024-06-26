#[doc = "Register `CLKGATE_CON24` reader"]
pub type R = crate::R<ClkgateCon24Spec>;
#[doc = "Register `CLKGATE_CON24` writer"]
pub type W = crate::W<ClkgateCon24Spec>;
#[doc = "Field `HCLK_ROM_EN` reader - hclk_rom clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkRomEnR = crate::BitReader;
#[doc = "Field `HCLK_ROM_EN` writer - hclk_rom clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkRomEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_M_CRYPTO0_EN` reader - hclk_m_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkMCrypto0EnR = crate::BitReader;
#[doc = "Field `HCLK_M_CRYPTO0_EN` writer - hclk_m_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkMCrypto0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_S_CRYPTO0_EN` reader - hclk_s_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSCrypto0EnR = crate::BitReader;
#[doc = "Field `HCLK_S_CRYPTO0_EN` writer - hclk_s_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSCrypto0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_M0_PERILP_EN` reader - sclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
pub type SclkM0PerilpEnR = crate::BitReader;
#[doc = "Field `SCLK_M0_PERILP_EN` writer - sclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
pub type SclkM0PerilpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_M0_PERILP_EN` reader - hclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkM0PerilpEnR = crate::BitReader;
#[doc = "Field `HCLK_M0_PERILP_EN` writer - hclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkM0PerilpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCLK_M0_PERILP_EN` reader - dclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
pub type DclkM0PerilpEnR = crate::BitReader;
#[doc = "Field `DCLK_M0_PERILP_EN` writer - dclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
pub type DclkM0PerilpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_M0_PERILP_DEC_EN` reader - clk_m0_perilp_dec clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkM0PerilpDecEnR = crate::BitReader;
#[doc = "Field `CLK_M0_PERILP_DEC_EN` writer - clk_m0_perilp_dec clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkM0PerilpDecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERILP_SGRF_EN` reader - pclk_perilp_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkPerilpSgrfEnR = crate::BitReader;
#[doc = "Field `PCLK_PERILP_SGRF_EN` writer - pclk_perilp_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkPerilpSgrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_M_CRYPTO1_EN` reader - hclk_m_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkMCrypto1EnR = crate::BitReader;
#[doc = "Field `HCLK_M_CRYPTO1_EN` writer - hclk_m_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkMCrypto1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_S_CRYPTO1_EN` reader - hclk_s_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSCrypto1EnR = crate::BitReader;
#[doc = "Field `HCLK_S_CRYPTO1_EN` writer - hclk_s_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSCrypto1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4 - hclk_rom clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_rom_en(&self) -> HclkRomEnR {
        HclkRomEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hclk_m_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_m_crypto0_en(&self) -> HclkMCrypto0EnR {
        HclkMCrypto0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hclk_s_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_s_crypto0_en(&self) -> HclkSCrypto0EnR {
        HclkSCrypto0EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - sclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn sclk_m0_perilp_en(&self) -> SclkM0PerilpEnR {
        SclkM0PerilpEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_m0_perilp_en(&self) -> HclkM0PerilpEnR {
        HclkM0PerilpEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - dclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn dclk_m0_perilp_en(&self) -> DclkM0PerilpEnR {
        DclkM0PerilpEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_m0_perilp_dec clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_m0_perilp_dec_en(&self) -> ClkM0PerilpDecEnR {
        ClkM0PerilpDecEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - pclk_perilp_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn pclk_perilp_sgrf_en(&self) -> PclkPerilpSgrfEnR {
        PclkPerilpSgrfEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hclk_m_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_m_crypto1_en(&self) -> HclkMCrypto1EnR {
        HclkMCrypto1EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hclk_s_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_s_crypto1_en(&self) -> HclkSCrypto1EnR {
        HclkSCrypto1EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - hclk_rom clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_rom_en(&mut self) -> HclkRomEnW<ClkgateCon24Spec> {
        HclkRomEnW::new(self, 4)
    }
    #[doc = "Bit 5 - hclk_m_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_m_crypto0_en(&mut self) -> HclkMCrypto0EnW<ClkgateCon24Spec> {
        HclkMCrypto0EnW::new(self, 5)
    }
    #[doc = "Bit 6 - hclk_s_crypto0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_s_crypto0_en(&mut self) -> HclkSCrypto0EnW<ClkgateCon24Spec> {
        HclkSCrypto0EnW::new(self, 6)
    }
    #[doc = "Bit 8 - sclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_m0_perilp_en(&mut self) -> SclkM0PerilpEnW<ClkgateCon24Spec> {
        SclkM0PerilpEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_m0_perilp_en(&mut self) -> HclkM0PerilpEnW<ClkgateCon24Spec> {
        HclkM0PerilpEnW::new(self, 9)
    }
    #[doc = "Bit 10 - dclk_m0_perilp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_m0_perilp_en(&mut self) -> DclkM0PerilpEnW<ClkgateCon24Spec> {
        DclkM0PerilpEnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_m0_perilp_dec clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_m0_perilp_dec_en(&mut self) -> ClkM0PerilpDecEnW<ClkgateCon24Spec> {
        ClkM0PerilpDecEnW::new(self, 11)
    }
    #[doc = "Bit 13 - pclk_perilp_sgrf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perilp_sgrf_en(&mut self) -> PclkPerilpSgrfEnW<ClkgateCon24Spec> {
        PclkPerilpSgrfEnW::new(self, 13)
    }
    #[doc = "Bit 14 - hclk_m_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_m_crypto1_en(&mut self) -> HclkMCrypto1EnW<ClkgateCon24Spec> {
        HclkMCrypto1EnW::new(self, 14)
    }
    #[doc = "Bit 15 - hclk_s_crypto1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_s_crypto1_en(&mut self) -> HclkSCrypto1EnW<ClkgateCon24Spec> {
        HclkSCrypto1EnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon24Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon24Spec;
impl crate::RegisterSpec for ClkgateCon24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con24::R`](R) reader structure"]
impl crate::Readable for ClkgateCon24Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con24::W`](W) writer structure"]
impl crate::Writable for ClkgateCon24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON24 to value 0"]
impl crate::Resettable for ClkgateCon24Spec {
    const RESET_VALUE: u32 = 0;
}
