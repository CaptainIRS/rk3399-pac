#[doc = "Register `HDCPREG_RMLCTL` reader"]
pub type R = crate::R<HdcpregRmlctlSpec>;
#[doc = "Register `HDCPREG_RMLCTL` writer"]
pub type W = crate::W<HdcpregRmlctlSpec>;
#[doc = "Field `ODPK_DECRYPT_ENABLE` reader - When set (1'b1), this bit activates the decryption of the Device Private keys."]
pub type OdpkDecryptEnableR = crate::BitReader;
#[doc = "Field `ODPK_DECRYPT_ENABLE` writer - When set (1'b1), this bit activates the decryption of the Device Private keys."]
pub type OdpkDecryptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set (1'b1), this bit activates the decryption of the Device Private keys."]
    #[inline(always)]
    pub fn odpk_decrypt_enable(&self) -> OdpkDecryptEnableR {
        OdpkDecryptEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set (1'b1), this bit activates the decryption of the Device Private keys."]
    #[inline(always)]
    #[must_use]
    pub fn odpk_decrypt_enable(&mut self) -> OdpkDecryptEnableW<HdcpregRmlctlSpec> {
        OdpkDecryptEnableW::new(self, 0)
    }
}
#[doc = "When set (1'b1), this bit activates the decryption of the Device Private keys.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_rmlctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_rmlctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregRmlctlSpec;
impl crate::RegisterSpec for HdcpregRmlctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_rmlctl::R`](R) reader structure"]
impl crate::Readable for HdcpregRmlctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_rmlctl::W`](W) writer structure"]
impl crate::Writable for HdcpregRmlctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_RMLCTL to value 0"]
impl crate::Resettable for HdcpregRmlctlSpec {
    const RESET_VALUE: u8 = 0;
}
