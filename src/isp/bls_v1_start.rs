#[doc = "Register `BLS_V1_START` reader"]
pub type R = crate::R<BlsV1StartSpec>;
#[doc = "Register `BLS_V1_START` writer"]
pub type W = crate::W<BlsV1StartSpec>;
#[doc = "Field `BLS_V1_START` reader - Black pixel position\n\n"]
pub type BlsV1StartR = crate::FieldReader<u16>;
#[doc = "Field `BLS_V1_START` writer - Black pixel position\n\n"]
pub type BlsV1StartW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Black pixel position\n\n"]
    #[inline(always)]
    pub fn bls_v1_start(&self) -> BlsV1StartR {
        BlsV1StartR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Black pixel position\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_v1_start(&mut self) -> BlsV1StartW<BlsV1StartSpec> {
        BlsV1StartW::new(self, 0)
    }
}
#[doc = "window 1 vertical start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_v1_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_v1_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsV1StartSpec;
impl crate::RegisterSpec for BlsV1StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_v1_start::R`](R) reader structure"]
impl crate::Readable for BlsV1StartSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_v1_start::W`](W) writer structure"]
impl crate::Writable for BlsV1StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_V1_START to value 0"]
impl crate::Resettable for BlsV1StartSpec {
    const RESET_VALUE: u32 = 0;
}
