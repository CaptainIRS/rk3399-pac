#[doc = "Register `MAX_READS` reader"]
pub type R = crate::R<MaxReadsSpec>;
#[doc = "Register `MAX_READS` writer"]
pub type W = crate::W<MaxReadsSpec>;
#[doc = "Field `MAX_READS` reader - Limit the number of outstanding read transactions to this amount"]
pub type MaxReadsR = crate::FieldReader;
#[doc = "Field `MAX_READS` writer - Limit the number of outstanding read transactions to this amount"]
pub type MaxReadsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Limit the number of outstanding read transactions to this amount"]
    #[inline(always)]
    pub fn max_reads(&self) -> MaxReadsR {
        MaxReadsR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Limit the number of outstanding read transactions to this amount"]
    #[inline(always)]
    #[must_use]
    pub fn max_reads(&mut self) -> MaxReadsW<MaxReadsSpec> {
        MaxReadsW::new(self, 0)
    }
}
#[doc = "maximum read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_reads::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`max_reads::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxReadsSpec;
impl crate::RegisterSpec for MaxReadsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_reads::R`](R) reader structure"]
impl crate::Readable for MaxReadsSpec {}
#[doc = "`write(|w| ..)` method takes [`max_reads::W`](W) writer structure"]
impl crate::Writable for MaxReadsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX_READS to value 0x1c"]
impl crate::Resettable for MaxReadsSpec {
    const RESET_VALUE: u32 = 0x1c;
}
