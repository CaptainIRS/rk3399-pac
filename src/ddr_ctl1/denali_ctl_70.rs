#[doc = "Register `DENALI_CTL_70` reader"]
pub type R = crate::R<DenaliCtl70Spec>;
#[doc = "Register `DENALI_CTL_70` writer"]
pub type W = crate::W<DenaliCtl70Spec>;
#[doc = "Field `DFS_ZQ_EN` reader - Enables ZQ calibration during a DFS exit. Set to 1 to enable."]
pub type DfsZqEnR = crate::BitReader;
#[doc = "Field `DFS_ZQ_EN` writer - Enables ZQ calibration during a DFS exit. Set to 1 to enable."]
pub type DfsZqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_CALVL_EN` reader - Enables CA training during a DFS exit. Set to 1 to enable."]
pub type DfsCalvlEnR = crate::BitReader;
#[doc = "Field `DFS_CALVL_EN` writer - Enables CA training during a DFS exit. Set to 1 to enable."]
pub type DfsCalvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_WRLVL_EN` reader - Enables write leveling during a DFS exit. Set to 1 to enable."]
pub type DfsWrlvlEnR = crate::BitReader;
#[doc = "Field `DFS_WRLVL_EN` writer - Enables write leveling during a DFS exit. Set to 1 to enable."]
pub type DfsWrlvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_RDLVL_EN` reader - Enables read data eye training during a DFS exit. Set to 1 to enable."]
pub type DfsRdlvlEnR = crate::BitReader;
#[doc = "Field `DFS_RDLVL_EN` writer - Enables read data eye training during a DFS exit. Set to 1 to enable."]
pub type DfsRdlvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables ZQ calibration during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_zq_en(&self) -> DfsZqEnR {
        DfsZqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables CA training during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_calvl_en(&self) -> DfsCalvlEnR {
        DfsCalvlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables write leveling during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_wrlvl_en(&self) -> DfsWrlvlEnR {
        DfsWrlvlEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables read data eye training during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_rdlvl_en(&self) -> DfsRdlvlEnR {
        DfsRdlvlEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables ZQ calibration during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_zq_en(&mut self) -> DfsZqEnW<DenaliCtl70Spec> {
        DfsZqEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables CA training during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_calvl_en(&mut self) -> DfsCalvlEnW<DenaliCtl70Spec> {
        DfsCalvlEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables write leveling during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_wrlvl_en(&mut self) -> DfsWrlvlEnW<DenaliCtl70Spec> {
        DfsWrlvlEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables read data eye training during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_rdlvl_en(&mut self) -> DfsRdlvlEnW<DenaliCtl70Spec> {
        DfsRdlvlEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_70::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_70::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl70Spec;
impl crate::RegisterSpec for DenaliCtl70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_70::R`](R) reader structure"]
impl crate::Readable for DenaliCtl70Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_70::W`](W) writer structure"]
impl crate::Writable for DenaliCtl70Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_70 to value 0"]
impl crate::Resettable for DenaliCtl70Spec {
    const RESET_VALUE: u32 = 0;
}
