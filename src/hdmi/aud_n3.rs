#[doc = "Register `AUD_N3` reader"]
pub type R = crate::R<AudN3Spec>;
#[doc = "Register `AUD_N3` writer"]
pub type W = crate::W<AudN3Spec>;
#[doc = "Field `AUDN` reader - HDMI Audio Clock Regenerator N value"]
pub type AudnR = crate::FieldReader;
#[doc = "Field `AUDN` writer - HDMI Audio Clock Regenerator N value"]
pub type AudnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NCTS_ATOMIC_WRITE` reader - When set, the new N and CTS values are only used when aud_n1 register is written. If clear, N and CTS data is updated each time a new N or CTS byte is Bits Name Attr Description written. The following write sequence is recommended: aud_n3 (set bit ncts_atomic_write if desired) aud_cts3 (set CTS_manual and CTS value if desired/enabled) aud_cts2 (required in CTS_manual) aud_cts1 (required in CTS_manual) aud_n3 (bit ncts_atomic_write with same value as in step 1.) aud_n2 aud_n1 For dynamic N/CTS changes, perform only steps from 2-7 or 5-7 depending on the state of CTS_manual."]
pub type NctsAtomicWriteR = crate::BitReader;
#[doc = "Field `NCTS_ATOMIC_WRITE` writer - When set, the new N and CTS values are only used when aud_n1 register is written. If clear, N and CTS data is updated each time a new N or CTS byte is Bits Name Attr Description written. The following write sequence is recommended: aud_n3 (set bit ncts_atomic_write if desired) aud_cts3 (set CTS_manual and CTS value if desired/enabled) aud_cts2 (required in CTS_manual) aud_cts1 (required in CTS_manual) aud_n3 (bit ncts_atomic_write with same value as in step 1.) aud_n2 aud_n1 For dynamic N/CTS changes, perform only steps from 2-7 or 5-7 depending on the state of CTS_manual."]
pub type NctsAtomicWriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - HDMI Audio Clock Regenerator N value"]
    #[inline(always)]
    pub fn audn(&self) -> AudnR {
        AudnR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - When set, the new N and CTS values are only used when aud_n1 register is written. If clear, N and CTS data is updated each time a new N or CTS byte is Bits Name Attr Description written. The following write sequence is recommended: aud_n3 (set bit ncts_atomic_write if desired) aud_cts3 (set CTS_manual and CTS value if desired/enabled) aud_cts2 (required in CTS_manual) aud_cts1 (required in CTS_manual) aud_n3 (bit ncts_atomic_write with same value as in step 1.) aud_n2 aud_n1 For dynamic N/CTS changes, perform only steps from 2-7 or 5-7 depending on the state of CTS_manual."]
    #[inline(always)]
    pub fn ncts_atomic_write(&self) -> NctsAtomicWriteR {
        NctsAtomicWriteR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HDMI Audio Clock Regenerator N value"]
    #[inline(always)]
    #[must_use]
    pub fn audn(&mut self) -> AudnW<AudN3Spec> {
        AudnW::new(self, 0)
    }
    #[doc = "Bit 7 - When set, the new N and CTS values are only used when aud_n1 register is written. If clear, N and CTS data is updated each time a new N or CTS byte is Bits Name Attr Description written. The following write sequence is recommended: aud_n3 (set bit ncts_atomic_write if desired) aud_cts3 (set CTS_manual and CTS value if desired/enabled) aud_cts2 (required in CTS_manual) aud_cts1 (required in CTS_manual) aud_n3 (bit ncts_atomic_write with same value as in step 1.) aud_n2 aud_n1 For dynamic N/CTS changes, perform only steps from 2-7 or 5-7 depending on the state of CTS_manual."]
    #[inline(always)]
    #[must_use]
    pub fn ncts_atomic_write(&mut self) -> NctsAtomicWriteW<AudN3Spec> {
        NctsAtomicWriteW::new(self, 7)
    }
}
#[doc = "HDMI Audio Clock Regenerator N value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_n3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_n3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudN3Spec;
impl crate::RegisterSpec for AudN3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_n3::R`](R) reader structure"]
impl crate::Readable for AudN3Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_n3::W`](W) writer structure"]
impl crate::Writable for AudN3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_N3 to value 0"]
impl crate::Resettable for AudN3Spec {
    const RESET_VALUE: u8 = 0;
}
