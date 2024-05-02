#[doc = "Register `BLS_H2_START` reader"]
pub type R = crate::R<BlsH2StartSpec>;
#[doc = "Register `BLS_H2_START` writer"]
pub type W = crate::W<BlsH2StartSpec>;
#[doc = "Field `BLS_H2_START` reader - Black pixel position\n\n"]
pub type BlsH2StartR = crate::FieldReader<u16>;
#[doc = "Field `BLS_H2_START` writer - Black pixel position\n\n"]
pub type BlsH2StartW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Black pixel position\n\n"]
    #[inline(always)]
    pub fn bls_h2_start(&self) -> BlsH2StartR {
        BlsH2StartR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Black pixel position\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_h2_start(&mut self) -> BlsH2StartW<BlsH2StartSpec> {
        BlsH2StartW::new(self, 0)
    }
}
#[doc = "window 2 horizontal start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h2_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h2_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsH2StartSpec;
impl crate::RegisterSpec for BlsH2StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_h2_start::R`](R) reader structure"]
impl crate::Readable for BlsH2StartSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_h2_start::W`](W) writer structure"]
impl crate::Writable for BlsH2StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_H2_START to value 0"]
impl crate::Resettable for BlsH2StartSpec {
    const RESET_VALUE: u32 = 0;
}
