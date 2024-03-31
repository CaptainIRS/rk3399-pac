#[doc = "Register `CORE_PWR_ST` reader"]
pub type R = crate::R<CorePwrStSpec>;
#[doc = "l2flushdone status of cluster_l\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2flushdoneClusterL {
    #[doc = "0: cluster_l l2flushdone status is 0"]
    B0 = 0,
    #[doc = "1: cluster_l l2flushdone status is 1"]
    B1 = 1,
}
impl From<L2flushdoneClusterL> for bool {
    #[inline(always)]
    fn from(variant: L2flushdoneClusterL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2FLUSHDONE_CLUSTER_L` reader - l2flushdone status of cluster_l"]
pub type L2flushdoneClusterLR = crate::BitReader<L2flushdoneClusterL>;
impl L2flushdoneClusterLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2flushdoneClusterL {
        match self.bits {
            false => L2flushdoneClusterL::B0,
            true => L2flushdoneClusterL::B1,
        }
    }
    #[doc = "cluster_l l2flushdone status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2flushdoneClusterL::B0
    }
    #[doc = "cluster_l l2flushdone status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2flushdoneClusterL::B1
    }
}
#[doc = "standbywfil2 status of cluster_l\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standbywfil2ClusterL {
    #[doc = "0: cluster_l standbywfil2 status is 0"]
    B0 = 0,
    #[doc = "1: cluster_l standbywfil2 status is 1"]
    B1 = 1,
}
impl From<Standbywfil2ClusterL> for bool {
    #[inline(always)]
    fn from(variant: Standbywfil2ClusterL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBYWFIL2_CLUSTER_L` reader - standbywfil2 status of cluster_l"]
pub type Standbywfil2ClusterLR = crate::BitReader<Standbywfil2ClusterL>;
impl Standbywfil2ClusterLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Standbywfil2ClusterL {
        match self.bits {
            false => Standbywfil2ClusterL::B0,
            true => Standbywfil2ClusterL::B1,
        }
    }
    #[doc = "cluster_l standbywfil2 status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Standbywfil2ClusterL::B0
    }
    #[doc = "cluster_l standbywfil2 status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Standbywfil2ClusterL::B1
    }
}
#[doc = "standbywfe status of cluster_l\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StandbywfeClusterL {
    #[doc = "0: cluster_l standbywfe status is 0"]
    B0 = 0,
    #[doc = "1: cluster_l standbywfe status is 1"]
    B1 = 1,
}
impl From<StandbywfeClusterL> for u8 {
    #[inline(always)]
    fn from(variant: StandbywfeClusterL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StandbywfeClusterL {
    type Ux = u8;
}
#[doc = "Field `STANDBYWFE_CLUSTER_L` reader - standbywfe status of cluster_l"]
pub type StandbywfeClusterLR = crate::FieldReader<StandbywfeClusterL>;
impl StandbywfeClusterLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StandbywfeClusterL> {
        match self.bits {
            0 => Some(StandbywfeClusterL::B0),
            1 => Some(StandbywfeClusterL::B1),
            _ => None,
        }
    }
    #[doc = "cluster_l standbywfe status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StandbywfeClusterL::B0
    }
    #[doc = "cluster_l standbywfe status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StandbywfeClusterL::B1
    }
}
#[doc = "standbywfi status of cluster_l\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StandbywfiClusterL {
    #[doc = "0: cluster_l standbywfi status is 0"]
    B0 = 0,
    #[doc = "1: cluster_l standbywfi status is 1"]
    B1 = 1,
}
impl From<StandbywfiClusterL> for u8 {
    #[inline(always)]
    fn from(variant: StandbywfiClusterL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StandbywfiClusterL {
    type Ux = u8;
}
#[doc = "Field `STANDBYWFI_CLUSTER_L` reader - standbywfi status of cluster_l"]
pub type StandbywfiClusterLR = crate::FieldReader<StandbywfiClusterL>;
impl StandbywfiClusterLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StandbywfiClusterL> {
        match self.bits {
            0 => Some(StandbywfiClusterL::B0),
            1 => Some(StandbywfiClusterL::B1),
            _ => None,
        }
    }
    #[doc = "cluster_l standbywfi status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StandbywfiClusterL::B0
    }
    #[doc = "cluster_l standbywfi status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StandbywfiClusterL::B1
    }
}
#[doc = "l2flushdone status of cluster_b\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2flushdoneClusterB {
    #[doc = "0: cluster_b l2flushdone status is 0"]
    B0 = 0,
    #[doc = "1: cluster_b l2flushdone status is 1"]
    B1 = 1,
}
impl From<L2flushdoneClusterB> for bool {
    #[inline(always)]
    fn from(variant: L2flushdoneClusterB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2FLUSHDONE_CLUSTER_B` reader - l2flushdone status of cluster_b"]
pub type L2flushdoneClusterBR = crate::BitReader<L2flushdoneClusterB>;
impl L2flushdoneClusterBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2flushdoneClusterB {
        match self.bits {
            false => L2flushdoneClusterB::B0,
            true => L2flushdoneClusterB::B1,
        }
    }
    #[doc = "cluster_b l2flushdone status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2flushdoneClusterB::B0
    }
    #[doc = "cluster_b l2flushdone status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2flushdoneClusterB::B1
    }
}
#[doc = "standbywfil2 status of cluster_b\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standbywfil2ClusterB {
    #[doc = "0: cluster_b standbywfil2 status is 0"]
    B0 = 0,
    #[doc = "1: cluster_b standbywfil2 status is 1"]
    B1 = 1,
}
impl From<Standbywfil2ClusterB> for bool {
    #[inline(always)]
    fn from(variant: Standbywfil2ClusterB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBYWFIL2_CLUSTER_B` reader - standbywfil2 status of cluster_b"]
pub type Standbywfil2ClusterBR = crate::BitReader<Standbywfil2ClusterB>;
impl Standbywfil2ClusterBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Standbywfil2ClusterB {
        match self.bits {
            false => Standbywfil2ClusterB::B0,
            true => Standbywfil2ClusterB::B1,
        }
    }
    #[doc = "cluster_b standbywfil2 status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Standbywfil2ClusterB::B0
    }
    #[doc = "cluster_b standbywfil2 status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Standbywfil2ClusterB::B1
    }
}
#[doc = "standbywfe status of cluster_b\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StandbywfeClusterB {
    #[doc = "0: cluster_b standbywfe status is 0"]
    B0 = 0,
    #[doc = "1: cluster_b standbywfe status is 1"]
    B1 = 1,
}
impl From<StandbywfeClusterB> for u8 {
    #[inline(always)]
    fn from(variant: StandbywfeClusterB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StandbywfeClusterB {
    type Ux = u8;
}
#[doc = "Field `STANDBYWFE_CLUSTER_B` reader - standbywfe status of cluster_b"]
pub type StandbywfeClusterBR = crate::FieldReader<StandbywfeClusterB>;
impl StandbywfeClusterBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StandbywfeClusterB> {
        match self.bits {
            0 => Some(StandbywfeClusterB::B0),
            1 => Some(StandbywfeClusterB::B1),
            _ => None,
        }
    }
    #[doc = "cluster_b standbywfe status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StandbywfeClusterB::B0
    }
    #[doc = "cluster_b standbywfe status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StandbywfeClusterB::B1
    }
}
#[doc = "standbywfi status of cluster_b\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StandbywfiClusterB {
    #[doc = "0: cluster_b standbywfi status is 0"]
    B0 = 0,
    #[doc = "1: cluster_b standbywfi status is 1"]
    B1 = 1,
}
impl From<StandbywfiClusterB> for u8 {
    #[inline(always)]
    fn from(variant: StandbywfiClusterB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StandbywfiClusterB {
    type Ux = u8;
}
#[doc = "Field `STANDBYWFI_CLUSTER_B` reader - standbywfi status of cluster_b"]
pub type StandbywfiClusterBR = crate::FieldReader<StandbywfiClusterB>;
impl StandbywfiClusterBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StandbywfiClusterB> {
        match self.bits {
            0 => Some(StandbywfiClusterB::B0),
            1 => Some(StandbywfiClusterB::B1),
            _ => None,
        }
    }
    #[doc = "cluster_b standbywfi status is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StandbywfiClusterB::B0
    }
    #[doc = "cluster_b standbywfi status is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StandbywfiClusterB::B1
    }
}
#[doc = "Field `PACTIVE_CCI500` reader - CCI-500 P-channel active signal, active high"]
pub type PactiveCci500R = crate::FieldReader;
#[doc = "Field `PDENY_CCI500` reader - CCI-500 P-channel deny signal, active high"]
pub type PdenyCci500R = crate::BitReader;
#[doc = "Field `PACCEPT_CCI500` reader - CCI-500 P-channel accept signal, active high"]
pub type PacceptCci500R = crate::BitReader;
#[doc = "Field `QACTIVE_CCI500` reader - CCI-500 Q-channel active signal, active high"]
pub type QactiveCci500R = crate::BitReader;
#[doc = "Field `QDENY_CCI500` reader - CCI-500 Q-channel deny signal, active high"]
pub type QdenyCci500R = crate::BitReader;
#[doc = "Field `QACCEPTN_CCI500` reader - CCI-500 Q-channel accept signal, active low"]
pub type QacceptnCci500R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - l2flushdone status of cluster_l"]
    #[inline(always)]
    pub fn l2flushdone_cluster_l(&self) -> L2flushdoneClusterLR {
        L2flushdoneClusterLR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - standbywfil2 status of cluster_l"]
    #[inline(always)]
    pub fn standbywfil2_cluster_l(&self) -> Standbywfil2ClusterLR {
        Standbywfil2ClusterLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - standbywfe status of cluster_l"]
    #[inline(always)]
    pub fn standbywfe_cluster_l(&self) -> StandbywfeClusterLR {
        StandbywfeClusterLR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - standbywfi status of cluster_l"]
    #[inline(always)]
    pub fn standbywfi_cluster_l(&self) -> StandbywfiClusterLR {
        StandbywfiClusterLR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - l2flushdone status of cluster_b"]
    #[inline(always)]
    pub fn l2flushdone_cluster_b(&self) -> L2flushdoneClusterBR {
        L2flushdoneClusterBR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - standbywfil2 status of cluster_b"]
    #[inline(always)]
    pub fn standbywfil2_cluster_b(&self) -> Standbywfil2ClusterBR {
        Standbywfil2ClusterBR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - standbywfe status of cluster_b"]
    #[inline(always)]
    pub fn standbywfe_cluster_b(&self) -> StandbywfeClusterBR {
        StandbywfeClusterBR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - standbywfi status of cluster_b"]
    #[inline(always)]
    pub fn standbywfi_cluster_b(&self) -> StandbywfiClusterBR {
        StandbywfiClusterBR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:24 - CCI-500 P-channel active signal, active high"]
    #[inline(always)]
    pub fn pactive_cci500(&self) -> PactiveCci500R {
        PactiveCci500R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - CCI-500 P-channel deny signal, active high"]
    #[inline(always)]
    pub fn pdeny_cci500(&self) -> PdenyCci500R {
        PdenyCci500R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CCI-500 P-channel accept signal, active high"]
    #[inline(always)]
    pub fn paccept_cci500(&self) -> PacceptCci500R {
        PacceptCci500R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CCI-500 Q-channel active signal, active high"]
    #[inline(always)]
    pub fn qactive_cci500(&self) -> QactiveCci500R {
        QactiveCci500R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CCI-500 Q-channel deny signal, active high"]
    #[inline(always)]
    pub fn qdeny_cci500(&self) -> QdenyCci500R {
        QdenyCci500R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CCI-500 Q-channel accept signal, active low"]
    #[inline(always)]
    pub fn qacceptn_cci500(&self) -> QacceptnCci500R {
        QacceptnCci500R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "pmu core power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_pwr_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorePwrStSpec;
impl crate::RegisterSpec for CorePwrStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_pwr_st::R`](R) reader structure"]
impl crate::Readable for CorePwrStSpec {}
#[doc = "`reset()` method sets CORE_PWR_ST to value 0"]
impl crate::Resettable for CorePwrStSpec {
    const RESET_VALUE: u32 = 0;
}
