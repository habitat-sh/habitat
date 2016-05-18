module Habitat
  class PromotionError < StandardError
    def initialize(msg = 'Promotion of package on Depot server failed')
      super
    end
  end

  class UploadError < StandardError
    def initialize(msg = 'Upload of artifact to Depot server failed')
      super
    end
  end

  class DownloadError < StandardError
    def initialize(msg = 'Download of artifact from Depot server failed')
      super
    end
  end
end
