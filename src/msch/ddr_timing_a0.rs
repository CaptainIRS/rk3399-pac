#[doc = "Register `DdrTimingA0` reader"]
pub type R = crate::R<DdrTimingA0Spec>;
#[doc = "Register `DdrTimingA0` writer"]
pub type W = crate::W<DdrTimingA0Spec>;
#[doc = "Field `ACTTOACT` reader - Minimum number of scheduler clock cycles between two\n\nconsecutive DRAM Activate commands on the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRC / tCkG"]
pub type ActtoactR = crate::FieldReader;
#[doc = "Field `ACTTOACT` writer - Minimum number of scheduler clock cycles between two\n\nconsecutive DRAM Activate commands on the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRC / tCkG"]
pub type ActtoactW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RDTOMISS` reader - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(tRTP + tRP + tRCD –BL ×tCkD / 2) / tCkG"]
pub type RdtomissR = crate::FieldReader;
#[doc = "Field `RDTOMISS` writer - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(tRTP + tRP + tRCD –BL ×tCkD / 2) / tCkG"]
pub type RdtomissW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRTOMISS` reader - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(WL ×tCkD + tWR + tRP + tRCD) / tCkG"]
pub type WrtomissR = crate::FieldReader;
#[doc = "Field `WRTOMISS` writer - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(WL ×tCkD + tWR + tRP + tRCD) / tCkG"]
pub type WrtomissW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `READLATENCY` reader - Maximun delay between a read request and the first data response."]
pub type ReadlatencyR = crate::FieldReader;
#[doc = "Field `READLATENCY` writer - Maximun delay between a read request and the first data response."]
pub type ReadlatencyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Minimum number of scheduler clock cycles between two\n\nconsecutive DRAM Activate commands on the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRC / tCkG"]
    #[inline(always)]
    pub fn acttoact(&self) -> ActtoactR {
        ActtoactR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(tRTP + tRP + tRCD –BL ×tCkD / 2) / tCkG"]
    #[inline(always)]
    pub fn rdtomiss(&self) -> RdtomissR {
        RdtomissR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(WL ×tCkD + tWR + tRP + tRCD) / tCkG"]
    #[inline(always)]
    pub fn wrtomiss(&self) -> WrtomissR {
        WrtomissR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Maximun delay between a read request and the first data response."]
    #[inline(always)]
    pub fn readlatency(&self) -> ReadlatencyR {
        ReadlatencyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minimum number of scheduler clock cycles between two\n\nconsecutive DRAM Activate commands on the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRC / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn acttoact(&mut self) -> ActtoactW<DdrTimingA0Spec> {
        ActtoactW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(tRTP + tRP + tRCD –BL ×tCkD / 2) / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn rdtomiss(&mut self) -> RdtomissW<DdrTimingA0Spec> {
        RdtomissW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a new Read or Write command in another page\n\nof the same bank.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n(WL ×tCkD + tWR + tRP + tRCD) / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn wrtomiss(&mut self) -> WrtomissW<DdrTimingA0Spec> {
        WrtomissW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Maximun delay between a read request and the first data response."]
    #[inline(always)]
    #[must_use]
    pub fn readlatency(&mut self) -> ReadlatencyW<DdrTimingA0Spec> {
        ReadlatencyW::new(self, 24)
    }
}
#[doc = "DdrTimingA bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_timing_a0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_timing_a0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrTimingA0Spec;
impl crate::RegisterSpec for DdrTimingA0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_timing_a0::R`](R) reader structure"]
impl crate::Readable for DdrTimingA0Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_timing_a0::W`](W) writer structure"]
impl crate::Writable for DdrTimingA0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DdrTimingA0 to value 0x2814_0916"]
impl crate::Resettable for DdrTimingA0Spec {
    const RESET_VALUE: u32 = 0x2814_0916;
}
