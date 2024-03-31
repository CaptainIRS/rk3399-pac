#[doc = "Register `CHIP_ID_ADDR` reader"]
pub type R = crate::R<ChipIdAddrSpec>;
#[doc = "Register `CHIP_ID_ADDR` writer"]
pub type W = crate::W<ChipIdAddrSpec>;
#[doc = "Field `CHIP_ID` reader - 3399"]
pub type ChipIdR = crate::FieldReader<u32>;
#[doc = "Field `CHIP_ID` writer - 3399"]
pub type ChipIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 3399"]
    #[inline(always)]
    pub fn chip_id(&self) -> ChipIdR {
        ChipIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 3399"]
    #[inline(always)]
    #[must_use]
    pub fn chip_id(&mut self) -> ChipIdW<ChipIdAddrSpec> {
        ChipIdW::new(self, 0)
    }
}
#[doc = "chip id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chip_id_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_id_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChipIdAddrSpec;
impl crate::RegisterSpec for ChipIdAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_id_addr::R`](R) reader structure"]
impl crate::Readable for ChipIdAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`chip_id_addr::W`](W) writer structure"]
impl crate::Writable for ChipIdAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIP_ID_ADDR to value 0"]
impl crate::Resettable for ChipIdAddrSpec {
    const RESET_VALUE: u32 = 0;
}
