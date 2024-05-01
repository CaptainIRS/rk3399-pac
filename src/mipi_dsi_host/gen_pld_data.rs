#[doc = "Register `GEN_PLD_DATA` reader"]
pub type R = crate::R<GenPldDataSpec>;
#[doc = "Register `GEN_PLD_DATA` writer"]
pub type W = crate::W<GenPldDataSpec>;
#[doc = "Field `GEN_PLD_B1` reader - gen_pld_b1\n\nThis field indicates byte 1 of the packet payload."]
pub type GenPldB1R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B1` writer - gen_pld_b1\n\nThis field indicates byte 1 of the packet payload."]
pub type GenPldB1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_PLD_B2` reader - gen_pld_b2\n\nThis field indicates byte 2 of the packet payload."]
pub type GenPldB2R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B2` writer - gen_pld_b2\n\nThis field indicates byte 2 of the packet payload."]
pub type GenPldB2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_PLD_B3` reader - gen_pld_b3\n\nThis field indicates byte 3 of the packet payload."]
pub type GenPldB3R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B3` writer - gen_pld_b3\n\nThis field indicates byte 3 of the packet payload."]
pub type GenPldB3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_PLD_B4` reader - gen_pld_b4\n\nThis field indicates byte 4 of the packet payload."]
pub type GenPldB4R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B4` writer - gen_pld_b4\n\nThis field indicates byte 4 of the packet payload."]
pub type GenPldB4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - gen_pld_b1\n\nThis field indicates byte 1 of the packet payload."]
    #[inline(always)]
    pub fn gen_pld_b1(&self) -> GenPldB1R {
        GenPldB1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gen_pld_b2\n\nThis field indicates byte 2 of the packet payload."]
    #[inline(always)]
    pub fn gen_pld_b2(&self) -> GenPldB2R {
        GenPldB2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gen_pld_b3\n\nThis field indicates byte 3 of the packet payload."]
    #[inline(always)]
    pub fn gen_pld_b3(&self) -> GenPldB3R {
        GenPldB3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gen_pld_b4\n\nThis field indicates byte 4 of the packet payload."]
    #[inline(always)]
    pub fn gen_pld_b4(&self) -> GenPldB4R {
        GenPldB4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gen_pld_b1\n\nThis field indicates byte 1 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b1(&mut self) -> GenPldB1W<GenPldDataSpec> {
        GenPldB1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - gen_pld_b2\n\nThis field indicates byte 2 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b2(&mut self) -> GenPldB2W<GenPldDataSpec> {
        GenPldB2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - gen_pld_b3\n\nThis field indicates byte 3 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b3(&mut self) -> GenPldB3W<GenPldDataSpec> {
        GenPldB3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - gen_pld_b4\n\nThis field indicates byte 4 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b4(&mut self) -> GenPldB4W<GenPldDataSpec> {
        GenPldB4W::new(self, 24)
    }
}
#[doc = "Generic Payload Data In And Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_pld_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_pld_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenPldDataSpec;
impl crate::RegisterSpec for GenPldDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_pld_data::R`](R) reader structure"]
impl crate::Readable for GenPldDataSpec {}
#[doc = "`write(|w| ..)` method takes [`gen_pld_data::W`](W) writer structure"]
impl crate::Writable for GenPldDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_PLD_DATA to value 0"]
impl crate::Resettable for GenPldDataSpec {
    const RESET_VALUE: u32 = 0;
}
