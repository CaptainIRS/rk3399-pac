#[doc = "Register `SWREG_99` reader"]
pub type R = crate::R<Swreg99Spec>;
#[doc = "Register `SWREG_99` writer"]
pub type W = crate::W<Swreg99Spec>;
#[doc = "Field `MUTIMV_EN` reader - on-off flag for using exceed one mv every mb\n\non-off flag for using exceed one mv every mb"]
pub type MutimvEnR = crate::BitReader;
#[doc = "Field `MUTIMV_EN` writer - on-off flag for using exceed one mv every mb\n\non-off flag for using exceed one mv every mb"]
pub type MutimvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MV_4P_PLY` reader - 4p of differential MV penalty"]
pub type Mv4pPlyR = crate::FieldReader<u16>;
#[doc = "Field `MV_4P_PLY` writer - 4p of differential MV penalty"]
pub type Mv4pPlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MV_1P_4P_PLY` reader - 1p or 4p of differential MV penalty\n\nME. DMVPenaltyQp"]
pub type Mv1p4pPlyR = crate::FieldReader<u16>;
#[doc = "Field `MV_1P_4P_PLY` writer - 1p or 4p of differential MV penalty\n\nME. DMVPenaltyQp"]
pub type Mv1p4pPlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MV_1P_PLY` reader - 1p of differential MV penalty\n\ndifferential MV penalty for 1p"]
pub type Mv1pPlyR = crate::FieldReader<u16>;
#[doc = "Field `MV_1P_PLY` writer - 1p of differential MV penalty\n\ndifferential MV penalty for 1p"]
pub type Mv1pPlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - on-off flag for using exceed one mv every mb\n\non-off flag for using exceed one mv every mb"]
    #[inline(always)]
    pub fn mutimv_en(&self) -> MutimvEnR {
        MutimvEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - 4p of differential MV penalty"]
    #[inline(always)]
    pub fn mv_4p_ply(&self) -> Mv4pPlyR {
        Mv4pPlyR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - 1p or 4p of differential MV penalty\n\nME. DMVPenaltyQp"]
    #[inline(always)]
    pub fn mv_1p_4p_ply(&self) -> Mv1p4pPlyR {
        Mv1p4pPlyR::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - 1p of differential MV penalty\n\ndifferential MV penalty for 1p"]
    #[inline(always)]
    pub fn mv_1p_ply(&self) -> Mv1pPlyR {
        Mv1pPlyR::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - on-off flag for using exceed one mv every mb\n\non-off flag for using exceed one mv every mb"]
    #[inline(always)]
    #[must_use]
    pub fn mutimv_en(&mut self) -> MutimvEnW<Swreg99Spec> {
        MutimvEnW::new(self, 0)
    }
    #[doc = "Bits 1:10 - 4p of differential MV penalty"]
    #[inline(always)]
    #[must_use]
    pub fn mv_4p_ply(&mut self) -> Mv4pPlyW<Swreg99Spec> {
        Mv4pPlyW::new(self, 1)
    }
    #[doc = "Bits 11:20 - 1p or 4p of differential MV penalty\n\nME. DMVPenaltyQp"]
    #[inline(always)]
    #[must_use]
    pub fn mv_1p_4p_ply(&mut self) -> Mv1p4pPlyW<Swreg99Spec> {
        Mv1p4pPlyW::new(self, 11)
    }
    #[doc = "Bits 21:30 - 1p of differential MV penalty\n\ndifferential MV penalty for 1p"]
    #[inline(always)]
    #[must_use]
    pub fn mv_1p_ply(&mut self) -> Mv1pPlyW<Swreg99Spec> {
        Mv1pPlyW::new(self, 21)
    }
}
#[doc = "mv related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg99Spec;
impl crate::RegisterSpec for Swreg99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_99::R`](R) reader structure"]
impl crate::Readable for Swreg99Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_99::W`](W) writer structure"]
impl crate::Writable for Swreg99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_99 to value 0"]
impl crate::Resettable for Swreg99Spec {
    const RESET_VALUE: u32 = 0;
}
