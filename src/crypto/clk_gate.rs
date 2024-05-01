#[doc = "Register `CLK_GATE` reader"]
pub type R = crate::R<ClkGateSpec>;
#[doc = "Register `CLK_GATE` writer"]
pub type W = crate::W<ClkGateSpec>;
#[doc = "Field `RSA_CLK_GATE` reader - 0= RSA clock is gated\n\n1= RSA clock is not gated"]
pub type RsaClkGateR = crate::BitReader;
#[doc = "Field `RSA_CLK_GATE` writer - 0= RSA clock is gated\n\n1= RSA clock is not gated"]
pub type RsaClkGateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDES_CLK_GATE` reader - 0= TDES clock is gated\n\n1= TDES clock is not gated"]
pub type TdesClkGateR = crate::BitReader;
#[doc = "Field `TDES_CLK_GATE` writer - 0= TDES clock is gated\n\n1= TDES clock is not gated"]
pub type TdesClkGateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH_CLK_GATE` reader - 0= HASH clock is gated\n\n1= HASH clock is not gated"]
pub type HashClkGateR = crate::BitReader;
#[doc = "Field `HASH_CLK_GATE` writer - 0= HASH clock is gated\n\n1= HASH clock is not gated"]
pub type HashClkGateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_CLK_GATE` reader - 0= AES clock is gated\n\n1= AES clock is not gated"]
pub type AesClkGateR = crate::BitReader;
#[doc = "Field `AES_CLK_GATE` writer - 0= AES clock is gated\n\n1= AES clock is not gated"]
pub type AesClkGateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0= RSA clock is gated\n\n1= RSA clock is not gated"]
    #[inline(always)]
    pub fn rsa_clk_gate(&self) -> RsaClkGateR {
        RsaClkGateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0= TDES clock is gated\n\n1= TDES clock is not gated"]
    #[inline(always)]
    pub fn tdes_clk_gate(&self) -> TdesClkGateR {
        TdesClkGateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0= HASH clock is gated\n\n1= HASH clock is not gated"]
    #[inline(always)]
    pub fn hash_clk_gate(&self) -> HashClkGateR {
        HashClkGateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0= AES clock is gated\n\n1= AES clock is not gated"]
    #[inline(always)]
    pub fn aes_clk_gate(&self) -> AesClkGateR {
        AesClkGateR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0= RSA clock is gated\n\n1= RSA clock is not gated"]
    #[inline(always)]
    #[must_use]
    pub fn rsa_clk_gate(&mut self) -> RsaClkGateW<ClkGateSpec> {
        RsaClkGateW::new(self, 0)
    }
    #[doc = "Bit 1 - 0= TDES clock is gated\n\n1= TDES clock is not gated"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_clk_gate(&mut self) -> TdesClkGateW<ClkGateSpec> {
        TdesClkGateW::new(self, 1)
    }
    #[doc = "Bit 2 - 0= HASH clock is gated\n\n1= HASH clock is not gated"]
    #[inline(always)]
    #[must_use]
    pub fn hash_clk_gate(&mut self) -> HashClkGateW<ClkGateSpec> {
        HashClkGateW::new(self, 2)
    }
    #[doc = "Bit 3 - 0= AES clock is gated\n\n1= AES clock is not gated"]
    #[inline(always)]
    #[must_use]
    pub fn aes_clk_gate(&mut self) -> AesClkGateW<ClkGateSpec> {
        AesClkGateW::new(self, 3)
    }
}
#[doc = "Clock Gate Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkGateSpec;
impl crate::RegisterSpec for ClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gate::R`](R) reader structure"]
impl crate::Readable for ClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_gate::W`](W) writer structure"]
impl crate::Writable for ClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_GATE to value 0x0f"]
impl crate::Resettable for ClkGateSpec {
    const RESET_VALUE: u32 = 0x0f;
}
