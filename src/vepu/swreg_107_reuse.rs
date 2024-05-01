#[doc = "Register `SWREG_107_REUSE` reader"]
pub type R = crate::R<Swreg107ReuseSpec>;
#[doc = "Register `SWREG_107_REUSE` writer"]
pub type W = crate::W<Swreg107ReuseSpec>;
#[doc = "Field `MV_PLY_8X4_4X8` reader - Penalty for using 8x4 or 4x8 MV.\n\nPenalty for using 8x4 or 4x8 MV."]
pub type MvPly8x4_4x8R = crate::FieldReader<u16>;
#[doc = "Field `MV_PLY_8X4_4X8` writer - Penalty for using 8x4 or 4x8 MV.\n\nPenalty for using 8x4 or 4x8 MV."]
pub type MvPly8x4_4x8W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MV_PLY_8X8` reader - Penalty for using 8x8 MV\n\nPenalty for using 8x8 MV"]
pub type MvPly8x8R = crate::FieldReader<u16>;
#[doc = "Field `MV_PLY_8X8` writer - Penalty for using 8x8 MV\n\nPenalty for using 8x8 MV"]
pub type MvPly8x8W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MV_PLY_16X8_8X16` reader - Penalty for using 16x8 or 8x16 MV.\n\nPenalty for using 16x8 or 8x16 MV"]
pub type MvPly16x8_8x16R = crate::FieldReader<u16>;
#[doc = "Field `MV_PLY_16X8_8X16` writer - Penalty for using 16x8 or 8x16 MV.\n\nPenalty for using 16x8 or 8x16 MV"]
pub type MvPly16x8_8x16W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Penalty for using 8x4 or 4x8 MV.\n\nPenalty for using 8x4 or 4x8 MV."]
    #[inline(always)]
    pub fn mv_ply_8x4_4x8(&self) -> MvPly8x4_4x8R {
        MvPly8x4_4x8R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Penalty for using 8x8 MV\n\nPenalty for using 8x8 MV"]
    #[inline(always)]
    pub fn mv_ply_8x8(&self) -> MvPly8x8R {
        MvPly8x8R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Penalty for using 16x8 or 8x16 MV.\n\nPenalty for using 16x8 or 8x16 MV"]
    #[inline(always)]
    pub fn mv_ply_16x8_8x16(&self) -> MvPly16x8_8x16R {
        MvPly16x8_8x16R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Penalty for using 8x4 or 4x8 MV.\n\nPenalty for using 8x4 or 4x8 MV."]
    #[inline(always)]
    #[must_use]
    pub fn mv_ply_8x4_4x8(&mut self) -> MvPly8x4_4x8W<Swreg107ReuseSpec> {
        MvPly8x4_4x8W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Penalty for using 8x8 MV\n\nPenalty for using 8x8 MV"]
    #[inline(always)]
    #[must_use]
    pub fn mv_ply_8x8(&mut self) -> MvPly8x8W<Swreg107ReuseSpec> {
        MvPly8x8W::new(self, 10)
    }
    #[doc = "Bits 20:29 - Penalty for using 16x8 or 8x16 MV.\n\nPenalty for using 16x8 or 8x16 MV"]
    #[inline(always)]
    #[must_use]
    pub fn mv_ply_16x8_8x16(&mut self) -> MvPly16x8_8x16W<Swreg107ReuseSpec> {
        MvPly16x8_8x16W::new(self, 20)
    }
}
#[doc = "JPEG control regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_107_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_107_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg107ReuseSpec;
impl crate::RegisterSpec for Swreg107ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_107_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg107ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_107_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg107ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_107_REUSE to value 0"]
impl crate::Resettable for Swreg107ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
