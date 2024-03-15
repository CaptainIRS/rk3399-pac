#[doc = "Register `FC_AVIELB[%s]` reader"]
pub type R = crate::R<FcAvielbSpec>;
#[doc = "Register `FC_AVIELB[%s]` writer"]
pub type W = crate::W<FcAvielbSpec>;
#[doc = "Field `FC_AVIELB` reader - This register defines the AVI InfoFrame End of Left Bar value. For more information, refer to the CEA- 861-E specification."]
pub type FcAvielbR = crate::FieldReader;
#[doc = "Field `FC_AVIELB` writer - This register defines the AVI InfoFrame End of Left Bar value. For more information, refer to the CEA- 861-E specification."]
pub type FcAvielbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the AVI InfoFrame End of Left Bar value. For more information, refer to the CEA- 861-E specification."]
    #[inline(always)]
    pub fn fc_avielb(&self) -> FcAvielbR {
        FcAvielbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the AVI InfoFrame End of Left Bar value. For more information, refer to the CEA- 861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_avielb(&mut self) -> FcAvielbW<FcAvielbSpec> {
        FcAvielbW::new(self, 0)
    }
}
#[doc = "This register defines the AVI InfoFrame End of Left Bar value. For more information, refer to the CEA- 861-E specification.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avielb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avielb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAvielbSpec;
impl crate::RegisterSpec for FcAvielbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_avielb::R`](R) reader structure"]
impl crate::Readable for FcAvielbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_avielb::W`](W) writer structure"]
impl crate::Writable for FcAvielbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVIELB[%s]
to value 0"]
impl crate::Resettable for FcAvielbSpec {
    const RESET_VALUE: u8 = 0;
}
